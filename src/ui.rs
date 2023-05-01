use cursive::Cursive;
use cursive::CursiveRunnable;
use cursive::views::*;
use cursive::traits::*;

fn main() {
    let mut siv = cursive::default();
    
    siv.add_layer(ResizedView::with_fixed_size((30, 10), (Dialog::around(LinearLayout::vertical()
        .child(TextView::new("Print your password"))
        .child(DummyView)
        .child(DummyView)
        .child(EditView::new().secret().with_name("password"))
        .child(DummyView)
        .child(LinearLayout::horizontal().child(Button::new("Quit", Cursive::quit)).child(ResizedView::with_fixed_size((15, 0),DummyView)).child(Button::new("Ok", |x| {dont_right(x)})))))));
    siv.run();
}

fn dont_right(ui : &mut Cursive){
    ui.add_layer(ResizedView::with_fixed_size((30, 10), Dialog::new().content(TextView::new("Your password is wrong")).button("Ok", |c| {c.pop_layer();})))
}