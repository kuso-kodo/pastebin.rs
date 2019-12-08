pub mod paste;

use handlebars::Handlebars;
lazy_static! {
    static ref HANDLEBARS: Handlebars = {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_file("paste", "template/paste.hbs")
            .unwrap();
        handlebars
            .register_template_file("new_paste", "template/new_paste.hbs")
            .unwrap();
        handlebars
    };
}
