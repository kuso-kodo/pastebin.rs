pub mod paste;

use handlebars::Handlebars;
lazy_static! {
    static ref HANDLEBARS: Handlebars = {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_file("show", "template/show.hbs")
            .unwrap();
        handlebars
            .register_template_file("new", "template/new.hbs")
            .unwrap();
        handlebars
            .register_template_file("list", "template/list.hbs")
            .unwrap();
        handlebars
    };
}
