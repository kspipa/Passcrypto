use cursive::Cursive;
use cursive::views::*;
use cursive::traits::*;


pub fn start(siv : &mut Cursive, first_or_not : String) {
    siv.add_layer(ResizedView::with_fixed_size((30, 10), Dialog::around(LinearLayout::vertical()
        .child(TextView::new(first_or_not))
        .child(DummyView)
        .child(DummyView)
        .child(EditView::new().secret().with_name("password"))
        .child(DummyView)
        .child(LinearLayout::horizontal().child(Button::new("Quit", Cursive::quit)).child(ResizedView::with_fixed_size((15, 0),DummyView)).child(Button::new("Ok", |x| {right(x)}))))));
}

pub fn dont_right(ui : &mut Cursive){
    ui.add_layer(ResizedView::with_fixed_size((30, 10), Dialog::new().content(TextView::new("Your password is wrong")).button("Ok", |c| {c.pop_layer();})))
}
pub fn right(ui : &mut Cursive){
    let mut select = SelectView::<String>::new().on_submit(|x , c : &str| {get_compass(x)}).with_name("select").fixed_size((50, 5));
    let mut dialog = Dialog::around(LinearLayout::vertical().child(ResizedView::with_fixed_size((5, 2), Button::new("Write new", |g| {get_compass(g);}))).child(ResizedView::with_fixed_size((5, 2), Button::new("Delete", |g| {get_compass(g);})))).fixed_size((50, 100));
    select.call_on_name("select", |x : &mut SelectView| {x.add_all_str("".split(""))});
    ui.add_fullscreen_layer(Dialog::around(LinearLayout::horizontal().child(select).child(DummyView.fixed_size((140, 5))).child(dialog)));
}
pub fn get_compass(ui : &mut Cursive){
    ui.add_layer(ResizedView::with_fixed_size((50, 10), Dialog::around(LinearLayout::vertical()
        .child(LinearLayout::horizontal().child(TextView::new("Username")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().with_name("name").fixed_size((20 , 2))))
        .child(LinearLayout::horizontal().child(TextView::new("Password")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().with_name("passs").fixed_size((20 , 2))))
        .child(LinearLayout::horizontal().child(TextView::new("Source")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().with_name("source").fixed_size((20 , 2))))
        .child(DummyView)
        .child(LinearLayout::horizontal().child(Button::new("Quit", |v| {v.pop_layer();})).child(DummyView.fixed_size((35, 1))).child(Button::new("Ok", |x| {
    let username = x.call_on_name("name", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    let password = x.call_on_name("passs", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    let source = x.call_on_name("source", |b: &mut EditView| {b.get_content().to_string()}).unwrap();
    add_passwords_in_list(vec![format!("{}:{}:{}", username, password, source)], x);
    x.pop_layer();
    }))))));
}
fn add_passwords_in_list(passs : Vec<String>, ui : &mut Cursive){
    ui.call_on_name("select", |x : &mut SelectView| {x.add_all_str(passs)});
}