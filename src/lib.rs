fn serve_order() {}
mod back_of_house {
    fn cook_order() {}
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
}
