use crate::i_type::IAttributesTrait;
use crate::i_type::byte_array::CWriteIBytes;
use cairo_syntax_parser::CairoWriteSlice;
use std::fmt::{Result as FmtResult, Write};

pub trait INameTrait {
    fn name(&self) -> &str;
}

pub trait IFieldTrait {
    fn field(&self) -> &str;
}

pub trait ITyTrait {
    fn ty(&self) -> &str;
}

pub trait IFieldsTrait {
    type Field: IFieldTrait + ITyTrait;
    fn fields(&self) -> &[Self::Field];
    fn field_fields(&self) -> Vec<&str> {
        self.fields()
            .iter()
            .map(IFieldTrait::field)
            .collect::<Vec<&str>>()
    }
    fn field_tys(&self) -> Vec<&str> {
        self.fields().iter().map(ITyTrait::ty).collect()
    }
}

pub trait MetaDataTrait {
    fn cwrite_meta_data<W: Write>(&self, buf: &mut W) -> FmtResult;
}

impl<T> MetaDataTrait for T
where
    T: IAttributesTrait + INameTrait,
{
    fn cwrite_meta_data<W: Write>(&self, buf: &mut W) -> FmtResult {
        self.name().to_iserialized_bytes(buf)?;
        let attribute_count = self.iattributes().len();
        write!(buf, ", {attribute_count}")?;
        if attribute_count > 0 {
            buf.write_char(',')?;
            self.iattributes().cwrite_csv(buf)?;
        }
        Ok(())
    }
}
