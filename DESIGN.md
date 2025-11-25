# Introspection Questions

- Should ByteArrays be packed in `TypeDef`s?
  - Saves 30%–50% of felts (events have a max size of 300 data felts).
  - Model struct would not be ABI-decodable, only a span.
- Should there be a version available in the ABI (MD5, contract call, or event)?  
  If contract call, it could include block number to improve indexing.
- Do we allow updating of models or specific changes (renaming, retyping)?

---

# Packing Defs

## Unpacked

In the unpacked system Serde is used for all types

### Example: Create Table Event

In the unpacked system a create table event would look like this:

```rust
pub struct CreateTableWithColumns {
    #[key]
    pub id: felt252,
    pub name: ByteArray,
    pub attributes: Span<Attribute>,
    pub primary: PrimaryDef,
    pub columns: Span<ColumnDef>,
}
```

This would be encoded and decoded by Serde and all the relevant types (`Attribute`, `PrimaryDef`, `ColumnDef`…) would be contained in the ABI so could be read and deserialized by any system that can read the ABI.

### Table Felt Cost

For a table:

$$T = 10 + \sum_{t=1}^{n_t}A_i + \sum_{j=1}^{n_p}A_j +  \sum_{c=1}^{n_{cols}} \left( 7 + \sum_{k=1}^{n_{c}}A_k + TD_c \right)$$

Where:

- $n_t$ = number of table attributes
- $n_p$ = number of primary attributes
- $n_{cols}$ = number of columns
- $n_c$ = number of attributes for column $c$
- $A_i, A_j, A_k$ = felt cost per attribute (Table, primary, column)
- $TD_c$ = felts in the `TypeDef` for column $c$

Breakdown:

| Component          | Type              | Felts                          |
| ------------------ | ----------------- | ------------------------------ |
| Table Name         | `ByteArray`       | $3$                            |
| Table Attributes   | `Span<Attribute>` | $1 + \sum_{i=1}^{n_t}A_i$      |
| Primary Name       | `ByteArray`       | $3$                            |
| Primary Attributes | `Span<Attribute>` | $1 + \sum_{j=1}^{n_p}A_j$      |
| Primary TypeDef    | `PrimaryTypeDef`  | $1$                            |
| Columns            | `Span<ColumnDef>` | $1 + \sum_{c=1}^{n_{cols}}C_c$ |

#### Column Breakdown (per column)

| Component         | Type              | Felts                     |
| ----------------- | ----------------- | ------------------------- |
| Column id         | `felt252`         | $1$                       |
| Column Name       | `ByteArray`       | $3$                       |
| Column Attributes | `Span<Attribute>` | $1 + \sum_{k=1}^{n_c}A_k$ |
| Column TypeDef    | `TypeDef`         | $TD_c$                    |

### TypeDef Size

The High level `TypeDef` consists of an enum variant $V +$ optional data depending on the type.

- All types use a minimum of $1$ felt to declare the enum variant
- `Primitives`, `ByteArrays` and similar: $1$; variant felt only
- `EncodedTypes`: $2$; variant + encoding felt
- `Arrays`/`Felt252Dict`/`Options`/`Nullable`: $1 + TD_{inner}$; variant + inner type def
- `FixedArrays`: $2 + TD_{inner}$; variant + length + inner type defs
- `Tuples`: $2 + \sum_{p=0}^{parts} TD_{p}$; variant + length + inner type defs
- `Ref`/`Custom`: $2$; variant + type id/custom id
- `Structs`/`Enums`: see below

---

## Attribute

```rust
pub struct Attribute {
    pub id: felt252,
    pub data: Span<felt252>,
}
```

Felts per attribute:

$$A = 2 + D$$

Where $D$ = number of data felts in the attribute (id + span length + data felts)

For a span of attributes:

$$\text{Total} = 1 + \sum_{i=1}^{n}(2 + D_i)$$

Where $n$ = number of attributes in the span

---

## StructDef

$$S = 6 + \sum_{i=1}^{n_s}A_i + \sum_{m=1}^{n_m} \left(4 + \sum_{j=1}^{n_{m,j}}A_j + TD_m\right)$$

Where:

- $n_s$ = number of struct attributes
- $n_m$ = number of members
- $n_{m,j}$ = number of attributes for member $m$
- $A_i, A_j$ = felt cost per attribute Struct and member attributes
- $TD_m$ = felts in the `TypeDef` for member $m$

Breakdown:

| Component         | Type              | Felts                     |
| ----------------- | ----------------- | ------------------------- |
| Enum Variant      | `felt252`         | $1$                       |
| Struct Name       | `ByteArray`       | $3$                       |
| Struct Attributes | `Span<Attribute>` | $1 + \sum_{i=1}^{n_s}A_i$ |
| Members           | `Span<MemberDef>` | $1 + \sum_{m=1}^{n_m}M_m$ |

#### Member Breakdown (per member)

| Component         | Type              | Felts                         |
| ----------------- | ----------------- | ----------------------------- |
| Member Name       | `ByteArray`       | $3$                           |
| Member Attributes | `Span<Attribute>` | $1 + \sum_{j=1}^{n_{m,j}}A_j$ |
| Member TypeDef    | `TypeDef`         | $TD_m$                        |

---

## EnumDef

$$E = 6 + \sum_{i=1}^{n_e}A_i + \sum_{v=1}^{n_v} \left(5 + \sum_{j=1}^{n_{v,j}}A_j + TD_v\right)$$

Where:

- $n_e$ = number of enum attributes
- $n_v$ = number of variants
- $n_{v,j}$ = number of attributes for variant $v$
- $A_i, A_j$ = felt cost per attribute (typically 2 felts each)
- $TD_v$ = felts in the `TypeDef` for variant $v$

Breakdown:

| Component       | Type               | Felts                     |
| --------------- | ------------------ | ------------------------- |
| Enum Variant    | `felt252`          | $1$                       |
| Enum Name       | `ByteArray`        | $3$                       |
| Enum Attributes | `Span<Attribute>`  | $1 + \sum_{i=1}^{n_e}A_i$ |
| Variants        | `Span<VariantDef>` | $1 + \sum_{v=1}^{n_v}V_v$ |

#### Variant Breakdown (per variant)

| Component          | Type              | Felts                         |
| ------------------ | ----------------- | ----------------------------- |
| Variant selector   | `felt252`         | $1$                           |
| Variant Name       | `ByteArray`       | $3$                           |
| Variant Attributes | `Span<Attribute>` | $1 + \sum_{j=1}^{n_{v,j}}A_j$ |
| Variant TypeDef    | `TypeDef`         | $TD_v$                        |

---

## Packed Create Table Event

```rust
pub struct CreateTableWithColumns {
    #[key]
    pub id: felt252,
    pub data: Span<felt252>,
}
```

### Con: Decoder needed

ABI-based systems only see a span of felts and cannot decode without the correct libraries.

### Pro: far fewer felts.

Most ByteArrays become a single felt instead of three. Attributes become $1 + data$ felt each instead of $2 + data$.

---

## ByteArray Encoding

Serde encoding:

$$B = 3 + \lfloor B/31 \rfloor$$

`B` = number of bytes.

Structure:

| Component      | Encoding         | Felts                      |
| -------------- | ---------------- | -------------------------- |
| Full `byte31`  | `Span<byte31>`   | $1 + \lfloor B/31 \rfloor$ |
| Pending word   | `felt252`        | $1$                        |
| Pending length | `Bounded<0, 31>` | $1$                        |

### Bit Layout of byte31

| Byte index (big-endian) | Global bit range | Byte Bit Diagram<br/>`7 6 5 4 3 2 1 0` | Use in byte31 | Useability in felt252 |
| ----------------------- | ---------------- | -------------------------------------- | ------------- | --------------------- |
| 31 (MSB)                | 255…248          | `X X X X * O O O`                      | Unused        | Partially usable      |
| 30                      | 247…240          | `± ± ± ± ± ± ± ±`                      | Usable Byte   | All Bits useable      |
| 29…1                    | 239…8            | `± ± ± ± ± ± ± ±`                      | Usable Byte   | All Bits useable      |
| 0 (LSB)                 | 7…0              | `± ± ± ± ± ± ± ±`                      | Usable Byte   | All Bits useable      |

- `X`: Unusable Bit
- `*`: Partially useable bit in felt252
- `O`: Bit not used in byte31 but used in felt252
- `±`: Bit used in both byte31 and felt252

The reason the `251` bit or 3rd bit in the 31st byte is partially usable is due to the fact a felt252 can have a max value of $2^{251}+17*2^{192}$ which means that for values between $2^{251}$ and its max the `251` bit is `1`. In values above $17*2^{192}$ the `251` has to be `0` to avoid exceeding the max value.

This means there are three bits (`250`, `249`, `248`) are unused in byte31 but usable in felt252.
These can therefore be use to encode `ByteArray`s in $1+\lfloor B/31 \rfloor$ felts.

- `249`: `1` if it's the last felt in the ByteArray, `0` otherwise.
- `248`: `1` if it's an incomplete byte31 (≤30 bytes), `0` otherwise.
- If it's an incomplete byte31 Byte 30 contains the number of bytes (0-30).

#### Examples

31: 255-248
30: 247-240
29-1: 239-8|0: 7-0

|     | 31B: 255-248b<br/>`7 6 5 4 3 2 1 0` | 30B: 247-240b<br/>`7 6 5 4 3 2 1 0` | 9-1B: 239-8b<br/>`7 6 5 4 3 2 1 0` | 0B 7-0b<br/>`7 6 5 4 3 2 1 0` |
| --- | ----------------------------------- | ----------------------------------- | ---------------------------------- | ----------------------------- |
| 1   | `0 0 0 0 0 0 0 0`                   | `± ± ± ± ± ± ± ±`                   | `± ± ± ± ± ± ± ±`                  | `± ± ± ± ± ± ± ±`             |
| 2   | `0 0 0 0 0 0 1 0`                   | `± ± ± ± ± ± ± ±`                   | `± ± ± ± ± ± ± ±`                  | `± ± ± ± ± ± ± ±`             |
| 3   | `0 0 0 0 0 0 1 1`                   | `0 0 0 1 1 1 1 0`                   | `± ± ± ± ± ± ± ±`                  | `± ± ± ± ± ± ± ±`             |
| 4   | `0 0 0 0 0 0 1 1`                   | `0 0 0 0 0 0 0 1`                   | `0 0 0 0 0 0 0 0`                  | `± ± ± ± ± ± ± ±`             |
| 5   | `0 0 0 0 0 0 1 1`                   | `0 0 0 0 0 0 0 0`                   | `0 0 0 0 0 0 0 0`                  | `0 0 0 0 0 0 0 0`             |
| 6   | `0 0 0 0 0 0 0 1`                   | `0 0 0 1 1 1 1 0`                   | `± ± ± ± ± ± ± ±`                  | `± ± ± ± ± ± ± ±`             |

- Full `bytes31` followed by more → $b^{249}=0$, $b^{248}=0$
- Full `bytes31` not followed by more → $b^{249}=1$, $b^{248}=0$
- 30 bytes not followed by more → $b^{249}=1$, $b^{248}=1$, $B_{30} = 30$ (number of bytes)
- 1 byte not followed by more → $b^{249}=1$, $b^{248}=1$, $B_{30} = 1$
- Empty ByteArray not followed by more data → $b^{249}=1$, $b^{248}=1$, $B_{30} = 0$
- 30 bytes + more data (technically allowed by the encoding but pointless) → $b^{249}=0$, $b^{248}=1$, $B_{30} = 30$

Result: up to 31 bytes encoded in 1 felt vs 3 with Serde.

---

## Attribute Encoding

Attributes can be encoded on top of the ByteArray optimization, using $b^{249-248}, B_{30}$ the same and $b^{250}$ to indicate if the attribute has data or is key-only.

Use bit `250`:

- `0`: key-only attribute
- `1`: attribute has additional data (encoded ByteArray follows)

The struct becomes:

```rust
pub struct Attribute {
    pub id: Utf8String,
    pub data: Option<ByteArray>,
}
```

Felts:

$$A = 1 + D$$

Saves 1 felt per attribute and allows for names/id to be byte arrays.

---

## Approximate Savings

It’s hard to derive a number in savings due to the custom nature of the `TypeDef` but assuming names are a single felt (Which is true for the majority of cases):

$$T = 6 + \sum_{t=1}^{n_t}A_i + \sum_{j=1}^{n_p}A_j +  \sum_{c=1}^{n_{cols}} \left( 5 + \sum_{k=1}^{n_{c}}A_k + TD_c \right)$$
$$S = 4 + \sum_{i=1}^{n_s}A_i + \sum_{m=1}^{n_m} \left(2 + \sum_{j=1}^{n_{m,j}}A_j + TD_m\right)$$
$$E = 6 + \sum_{i=1}^{n_e}A_i + \sum_{v=1}^{n_v} \left(3 + \sum_{j=1}^{n_{v,j}}A_j + TD_v\right)$$

Savings: ~30%, more for attribute-heavy or complex structs/enums.
Useful where event data limit (300 felts) is tight and to reduce gas.

---

# Embedded Version/Features

A version and features for each contract either readable via a call function or emitted in an event. (If its a call function it could also include the deployed block number to help indexers).
