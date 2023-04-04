use super::code_writer::CodeWriter;
use super::rust_types_values::*;
use protobuf::descriptor::*;
use protobuf::descriptorx::*;
use Customize;

struct ExtGen<'a> {
    file: &'a FileDescriptorProto,
    root_scope: &'a RootScope<'a>,
    field: &'a FieldDescriptorProto,
    customize: Customize,
}

impl<'a> ExtGen<'a> {
    fn extendee_rust_name(&self) -> String {
        type_name_to_rust_relative(self.field.get_extendee(), self.file, true, self.root_scope)
    }

    fn repeated(&self) -> bool {
        match self.field.get_label() {
            FieldDescriptorProtoLabel::LabelRepeated => true,
            FieldDescriptorProtoLabel::LabelOptional => false,
            FieldDescriptorProtoLabel::LabelRequired => {
                panic!("required ext field: {}", self.field.get_name())
            }
        }
    }

    fn return_type_gen(&self) -> ProtobufTypeGen {
        if self.field.has_type_name() {
            let rust_name_relative = type_name_to_rust_relative(
                self.field.get_type_name(),
                self.file,
                true,
                self.root_scope,
            );
            match self.field.get_type() {
                FieldDescriptorProtoType::TypeMessage => {
                    ProtobufTypeGen::Message(rust_name_relative)
                }
                FieldDescriptorProtoType::TypeEnum => ProtobufTypeGen::Enum(rust_name_relative),
                t => panic!("unknown type: {:?}", t),
            }
        } else {
            ProtobufTypeGen::Primitive(self.field.get_type(), PrimitiveTypeVariant::Default)
        }
    }

    fn write(&self, w: &mut CodeWriter) {
        let suffix = if self.repeated() {
            "Repeated"
        } else {
            "Optional"
        };
        let field_type = format!("::protobuf::ext::ExtField{}", suffix);
        w.pub_const(
            &self.field.rust_name(),
            &format!(
                "{}<{}, {}>",
                field_type,
                self.extendee_rust_name(),
                self.return_type_gen().rust_type(&self.customize),
            ),
            &format!(
                "{} {{ field_number: {}, phantom: ::std::marker::PhantomData }}",
                field_type,
                self.field.get_number()
            ),
        );
    }
}

pub(crate) fn write_extensions(
    file: &FileDescriptorProto,
    root_scope: &RootScope,
    w: &mut CodeWriter,
    customize: &Customize,
) {
    if file.get_extension().is_empty() {
        return;
    }

    w.write_line("");
    w.pub_mod("exts", |w| {
        w.write_line("use protobuf::Message as Message_imported_for_functions;");

        for field in file.get_extension() {
            if field.get_type() == FieldDescriptorProtoType::TypeGroup {
                continue;
            }

            w.write_line("");
            ExtGen {
                file: file,
                root_scope: root_scope,
                field: field,
                customize: customize.clone(),
            }
            .write(w);
        }
    });
}
