macro_rules! biggest {
    ($first: expr, $second: expr, $($rest: expr),*) => {
        if $first > $second {
            biggest!($first, $($rest),+)
        } else {
            biggest!($second, $($rest),+)
        }
    };
    ($first: expr, $second: expr) => {
        if $first > $second {
            $first
        } else {
            $second
        }
    };
    ($first: expr) => {
        $first
    };
    () => {
        0
    };
}
macro_rules! input_or_true {
    () => {
        true
    };
    ($expr: expr) => {
        $expr
    };
}
macro_rules! counter {
    () => {
        1
    };
    ($num: expr) => {
        $num + 1
    };
}
#[macro_export]
macro_rules! access_array {
    ($array: ident, $($param: ty),+) => {
        {
            let mut index = 0;
            $( {

                let temp = Into::<$param>::into($array[index]);
                index += 1;
                temp
            }
            ),*
        }
    };
}
#[macro_export]
macro_rules! access_single {
    ($index: tt, $array: tt) => {{
        let temp = $array[$index];
        index += 1;
        temp
    }};
}
pub fn access_single<T>(count: &mut usize, array: &[T]) -> T
where
    T: Clone,
{
    let temp = array[*count].clone();
    *count += 1;
    temp
}
#[macro_export]
macro_rules! call_with {
    ($array: ident, $func: ident, ($($param: ty),+)) => {
        let mut count = 0;
        $func(
            $({
                    let temp = TryInto::<$param>::try_into($array[count]);
                    count += 1;
                    temp?
            }),*
        )
    };
}
#[macro_export]
macro_rules! functions {
    ($lib: ident: $lib_namespace: ident; [$($import: ty: $import_namespace: ident),*]; [$($func_name: ident: $func: ident($($param: ty),*) $(;$is_const: expr)?),+]) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone)]
        pub enum $lib
        {
            $($func_name),+,
            $($import_namespace($import)),*
        }
        impl $crate::function::Function<$lib> for $lib
        {
            const NAMESPACE: &'static str = stringify!($lib_namespace);
            const MAX_ARGS: usize = biggest!($($arg_count),+);
            fn from_string(
                namespaces: &[&str],
                identifier: &str,
            ) -> Result<$lib, $crate::error::Error> {
                match namespaces {
                    [namespace, ..] => match *namespace {
                        $(<$import>::NAMESPACE => $lib::$import_namespace(<$import>::from_string(namespaces, identifier)?),)*
                        Self::NAMESPACE => Self::from_string(namespaces, identifier),
                        _ => Err($crate::error::Error::InvalidNamespace)
                    }
                    [] => match identifier {
                        $(stringify!($func) => Ok($lib::$func_name),)+
                        _ => Err($crate::error::Error::UnknownFunction)
                    }
                }

            }
            fn call(&self, args: &[$crate::value::Value]) -> Result<$crate::value::Value, $crate::error::Error> {
                match self {
                    $($lib::$func_name => { if args.len() == $arg_count {$func(args.try_into()?)} else { return Err($crate::error::Error::InvalidArgs)}},)+
                    $($lib::$import_namespace(i) => i.call(args)?,)*
                }
            }
            fn is_const(&self) -> bool {
                match self {
                    $($lib::$func_name => input_or_true!($($is_const)?),)*
                    $($lib::$import_namespace(i) => i.is_const(),)*
                }
            }
        }
    }
}
