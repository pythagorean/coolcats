#[macro_export]
macro_rules! class_names {
    (
        $name:literal
    ) => {
        $name
    };

    (
        $name:literal => $test:expr
    ) => {
        if $test { $name } else { "" }
    };

    (
        $testname:expr
    ) => {
        if $testname { stringify!{$testname} } else { "" }
    };

    (
        $name1:literal, $name2:literal, $($name3:literal => $test3:expr),+
    ) => {
        [  class_names!($name1),
           class_names!($name2),
         $(class_names!($name3 => $test3),)+].join(" ")
    };

    (
        $name1:literal, $($name2:literal => $test2:expr),+
    ) => {
        [  class_names!($name1),
         $(class_names!($name2 => $test2),)+].join(" ")
    };

    (
        $name1:literal, $name2:literal => $test2:expr, $($testname3:expr),+
    ) => {
        [  class_names!($name1),
           class_names!($name2 => $test2),
         $(class_names!($testname3),)+].join(" ")
    };

    (
        $($name:literal => $test:expr),+
    ) => {
        [$(class_names!($name => $test),)+].join(" ")
    };

    (
        $name1:literal => $test1:expr, $name2:literal => $test2:expr, $($testname3:expr),+
    ) => {
        [  class_names!($name1 => $test1),
           class_names!($name2 => $test2),
         $(class_names!($testname3),)+].join(" ")
    };

    (
        $name1:literal => $test1:expr, $($testname2:expr),+
    ) => {
        [  class_names!($name1 => $test1),
         $(class_names!($testname2),)+].join(" ")
    };

    (
        $($argv: tt),+
    ) => {
        [$(class_names!($argv),)+].join(" ")
    };
}
