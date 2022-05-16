use nom::IResult;
use nom::{bytes::complete::take, error::ErrorKind};

static mut VERSION_SUM: usize = 0;

#[derive(Debug)]
enum PacketTypeID {
    Literal(usize),
    Operator(OpType),
}

#[derive(Debug, PartialEq, Eq)]
enum OpType {
    Literal,
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

impl From<usize> for OpType {
    fn from(i: usize) -> Self {
        match i {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Min,
            3 => Self::Max,
            5 => Self::Gt,
            6 => Self::Lt,
            7 => Self::Eq,
            _ => {
                unreachable!()
            }
        }
    }
}

impl From<usize> for PacketTypeID {
    fn from(i: usize) -> Self {
        match i {
            4 => Self::Literal(4),
            n => Self::Operator(n.into()),
        }
    }
}

#[derive(Debug)]
struct Expr {
    op: OpType,
    value: usize,
    args: Option<Vec<Expr>>,
}

impl Expr {
    fn new(op: OpType, value: usize, args: Option<Vec<Expr>>) -> Self {
        Self { op, value, args }
    }
}

fn main() {
    let s = include_str!("input");
    let mut binary = "".to_owned();
    s.chars().for_each(|c| {
        let d = c.to_digit(16).unwrap();
        let b = format!("{:0>4b}", d);
        binary.push_str(&b);
    });

    let (_, root_expr) = parse_packet(binary.as_str()).unwrap();
    println!("part1: {}", unsafe { VERSION_SUM });
    println!("part2: {}", eval_expr(&root_expr.unwrap()));
}

fn parse_packet(input: &str) -> Result<(&str, Option<Expr>), Box<dyn std::error::Error + '_>> {
    if input.is_empty() {
        return Ok(("", None));
    }
    let mut pv_pt_parser = nom::sequence::tuple((take3, take3));
    let (input, (packet_version, packet_type_id)) = pv_pt_parser(input).unwrap();
    unsafe { VERSION_SUM += packet_version };

    match packet_type_id.into() {
        PacketTypeID::Literal(_) => parse_literal(input),
        PacketTypeID::Operator(o) => parse_operator(input, o),
    }
}

fn parse_literal(input: &str) -> Result<(&str, Option<Expr>), Box<dyn std::error::Error + '_>> {
    if input.is_empty() {
        return Ok(("", None));
    }
    let (mut input, mut len_type_id) = take::<_, _, (&str, ErrorKind)>(5u8)(input)?;
    let mut value = "".to_string();
    loop {
        value.push_str(&len_type_id[1..]);
        if len_type_id.starts_with('0') {
            break;
        }
        (input, len_type_id) = take::<_, _, (&str, ErrorKind)>(5u8)(input)?;
    }
    let v = usize::from_str_radix(&value, 2)?;
    Ok((input, Some(Expr::new(OpType::Literal, v, None))))
}

fn parse_operator(
    input: &str,
    op: OpType,
) -> Result<(&str, Option<Expr>), Box<dyn std::error::Error + '_>> {
    let (input, len_type_id) = take::<_, _, (&str, ErrorKind)>(1u8)(input)?;
    let len_type_id = usize::from_str_radix(len_type_id, 2)?;
    let len_of_sub_packet_bits: usize = if len_type_id == 0 { 15 } else { 11 };

    let (mut input, len_of_sub_packet) =
        take::<_, _, (&str, ErrorKind)>(len_of_sub_packet_bits)(input)?;

    let len_of_sub_packet = usize::from_str_radix(len_of_sub_packet, 2)?;
    let mut expr = Expr::new(op, 0, None);
    let mut args = vec![];
    if len_type_id == 0 {
        let (input, mut sub_packet) = take::<_, _, (&str, ErrorKind)>(len_of_sub_packet)(input)?;
        while !sub_packet.is_empty() {
            let (s, sub_expr) = parse_packet(sub_packet)?;
            if let Some(sub) = sub_expr {
                args.push(sub);
            }
            sub_packet = s;
        }
        expr.args = Some(args);
        Ok((input, Some(expr)))
    } else {
        for _ in 0..len_of_sub_packet {
            let (s, expr) = parse_packet(input)?;
            if let Some(sub) = expr {
                args.push(sub);
            }
            input = s;
        }
        expr.args = Some(args);
        Ok((input, Some(expr)))
    }
}

fn eval_expr(root: &Expr) -> usize {
    let mut arg_value = vec![];
    if let Some(args) = &root.args {
        for arg in args {
            arg_value.push(eval_expr(arg));
        }
    }
    match root.op {
        OpType::Sum => arg_value.iter().sum(),
        OpType::Product => arg_value.iter().product(),
        OpType::Min => *arg_value.iter().min().unwrap(),
        OpType::Max => *arg_value.iter().max().unwrap(),
        OpType::Gt => (arg_value[0] > arg_value[1]) as usize,
        OpType::Lt => (arg_value[0] < arg_value[1]) as usize,
        OpType::Eq => (arg_value[0] == arg_value[1]) as usize,
        OpType::Literal => root.value,
    }
}

fn take3(input: &str) -> IResult<&str, usize> {
    match take(3u8)(input) {
        Ok((i, o)) => {
            let o = usize::from_str_radix(o, 2)
                .map_err(|_e| nom::Err::Error(nom::error::Error::new(o, ErrorKind::Fail)))?;
            Ok((i, o))
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn bool_usize() {
        assert_eq!(0, (1 == 2) as usize);
        assert_eq!(0, (1 > 2) as usize);
        assert_eq!(1, (3 > 2) as usize);
        assert_eq!(1, (1 == 1) as usize);
    }
}
