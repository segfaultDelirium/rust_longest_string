#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn get_longest<'a>(strings: &'a [&str]) -> &'a str {
    let (best_index, _best_len) = strings
        .into_iter()
        .enumerate()
        .map(|(i, s)| (i, s.len()))
        .reduce(|(old_i, best_len), (i, len)| {
            if len > best_len {
                (i, len)
            } else {
                (old_i, best_len)
            }
        })
        .unwrap();

    strings[best_index]

    // ""
}

fn get_longest_string<'a>(strings: &'a [&str]) -> Option<&'a str> {
    let res =
        strings
            .into_iter()
            .map(|s| (s, s.len()))
            .reduce(|(longest, longest_len), (s, len)| {
                if len > longest_len {
                    (s, len)
                } else {
                    (longest, longest_len)
                }
            });
    if res.is_none() {
        None
    } else {
        Some(res.unwrap().0)
    }
}

fn main() {
    let x = 19;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(x);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("x is NOT borrowed in {:?}", number);

    let strings = vec!["hi", "world", "hel", "marcus", "sdfsdfsfds", "g"];
    let longest_string = get_longest(&strings);
    println!("strings: {:?}", strings);
    println!("longest_string: {}", longest_string);

    let also_longest_string = get_longest_string(&strings);
    println!("also_longest_string: {:?}", also_longest_string);

    println!("bye!");
}
