pub fn class_names(classes: &[(&str, bool)]) -> String {
    let names: Vec<&str> = classes
        .into_iter()
        .map(|(name, show)| {
            if *show {
                name
            } else {
                ""
            }
        })
        .collect();
    names.join(" ").into()
}
