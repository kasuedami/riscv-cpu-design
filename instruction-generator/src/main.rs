const FILE_HEADER: &str = "Ins[32] RD[5] RS1[5] RS2[5]\n";

fn main() {
    let data = all();
    let file_content = generate_file_contents(&data);
    println!("{}", file_content);
}

struct Instruction {
    data: u32,
    rd: u8,
    rs1: u8,
    rs2: u8,
}

fn all() -> Vec<Instruction> {
    let mut instructions = Vec::with_capacity(32 * 32 * 32);

    for rd in 0..32 {
        for rs1 in 0..32 {
            for rs2 in 0..32 {
                instructions.push(add(rd, rs1, rs2));
            }
        }
    }

    instructions
}

fn add(rd: u8, rs1: u8, rs2: u8) -> Instruction {
    assert!(rd < 32);
    assert!(rs1 < 32);
    assert!(rs2 < 32);

    let opcode = 0b0110011;
    let funct3 = 0b000 << 12;
    let funct7 = 0b0000000 << 23;
    let rd_shift = (rd as u32) << 7;
    let rs1_shift = (rs1 as u32) << 15;
    let rs2_shift = (rs2 as u32) << 20;

    let data = funct7 | rs2_shift | rs1_shift | funct3 | rd_shift | opcode;

    Instruction { data, rd, rs1, rs2 }
}

fn generate_file_contents(data: &[Instruction]) -> String {
    let mut content = FILE_HEADER.to_owned();

    for ins in data {
        content.push_str(&format!(
            "0x{:0>8x} {} {} {}\n",
            ins.data, ins.rd, ins.rs1, ins.rs2
        ));
    }

    content
}
