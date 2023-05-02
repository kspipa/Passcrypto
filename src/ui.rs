use cursive::Cursive;
use cursive::CursiveRunnable;
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
    let mut select = SelectView::<String>::new().on_submit(|x , c : &str| {get_compass(x)}).with_name("select").fixed_size((10, 5));
    select.call_on_name("select", |x : &mut SelectView| {x.add_all_str("select".split(""))});
    ui.add_layer(ResizedView::with_fixed_size((200, 200), Dialog::around(LinearLayout::horizontal()
    .child(select)
    .child(DummyView))));
}
pub fn get_compass(ui : &mut Cursive){
    ui.add_layer(ResizedView::with_fixed_size((50, 10), Dialog::around(LinearLayout::vertical()
        .child(LinearLayout::horizontal().child(TextView::new("Username")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().fixed_size((20 , 2)).with_name("name")))
        .child(LinearLayout::horizontal().child(TextView::new("Password")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().fixed_size((20 , 2)).with_name("passs")))
        .child(LinearLayout::horizontal().child(TextView::new("Source")).child(DummyView).child(TextView::new(":")).child(DummyView).child(EditView::new().fixed_size((20 , 2)).with_name("source")))
        .child(DummyView)
        .child(LinearLayout::horizontal().child(Button::new("Quit", |v| {v.pop_layer();})).child(ResizedView::with_fixed_size((35 , 1), DummyView)).child(Button::new("Ok", |v| {add_passwords_in_list(vec![_get_data_EditView(v)], v)}))))));
}
fn _get_data_EditView(ui : &mut Cursive) -> String{
    let username = &mut String::new();
    let password = &mut String::new();
    let source = &mut String::new();
    ui.call_on_name("name", |x : &mut EditView| {*username = x.get_content().to_string();});
    ui.call_on_name("passs", |x : &mut EditView| {*password = x.get_content().to_string();});
    ui.call_on_name("source", |x : &mut EditView| {*source = x.get_content().to_string();});
    ui.add_layer(Dialog::around(TextView::new(source.to_string())));
    let all = format!("{}:{}:{}", username, password, source);
    return all;
}
fn add_passwords_in_list(passs : Vec<String>, ui : &mut Cursive){
    ui.call_on_name("select", |x : &mut SelectView| {x.add_all_str(passs)});
}