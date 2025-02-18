use crate::class_info;
use crate::my_string;
#[derive(Debug)]
pub struct ObjectInfo {
    pub class_name: String,
    pub name: String,
    pub import_class_name: String,
}

impl ObjectInfo {
    pub fn new(
        class_name: &String,
        input_define: String,
        file_name: &String,
    ) -> Option<ObjectInfo> {
        let define = input_define.replace(" ", "");
        let name = match my_string::get_first_token(&define, '=') {
            Some(token) => token,
            None => return None,
        };
        let mut import_class_name = match my_string::cut(&define, '=', '(') {
            Some(token) => token,
            None => return None,
        };
        import_class_name = match import_class_name.find(".") {
            None => class_info::ClassInfo::add_file_profix(&file_name, &import_class_name, false),
            Some(_x) => import_class_name.replace(".", "_"),
        };
        return Some(ObjectInfo {
            class_name: class_name.clone(),
            name: name,
            import_class_name: import_class_name,
        });
    }
    pub fn new_object_fn(&self) -> String {
        let mut new_object_fn = String::new();
        /* do not generate new object for pikaMain class */
        let new_fn = format!(
            "    obj_newObj(self, \"{}\", \"{}\", New_{});\n",
            self.name, self.import_class_name, self.import_class_name
        );
        let module_define = format!(
            "#ifndef PIKA_MODULE_{}_DISABLE\n",
            self.import_class_name.to_ascii_uppercase()
        );
        new_object_fn.push_str(&module_define);
        new_object_fn.push_str(&new_fn);
        new_object_fn.push_str("#endif\n");
        return new_object_fn;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_info() {
        assert_eq!(
            ObjectInfo::new(
                &String::from("Test"),
                String::from("test=ImportTest()"),
                &"Pkg".to_string()
            )
            .unwrap()
            .import_class_name,
            String::from("Pkg_ImportTest")
        );
        assert_eq!(
            ObjectInfo::new(
                &String::from("Test"),
                String::from("test=ImportTest()"),
                &"Pkg".to_string()
            )
            .unwrap()
            .name,
            String::from("test")
        );
        assert_eq!(
            ObjectInfo::new(
                &String::from("Test"),
                String::from("test=ImportTest()"),
                &"Pkg".to_string()
            )
            .unwrap()
            .class_name,
            String::from("Test")
        );
    }
}
