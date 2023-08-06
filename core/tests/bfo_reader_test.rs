use bffcore::constants::constants::INSTRUCTION_COUNT;
use bffcore::constants::instructions::Instruction;

#[test]
fn instruction_conversion_integrity_check(){
    assert_eq!(INSTRUCTION_COUNT, 95);


    for i in 0..INSTRUCTION_COUNT as u8 {
        let raw = [i, 0, 0, 0, 0, 0, 0, 0];
        let instruction = Instruction::from_bfo_bytes(raw);
        let bytes  = instruction.to_bfo_bytes();

        assert_eq!(raw[0], bytes[0]);
        assert_eq!(raw[1], bytes[1]);
        assert_eq!(raw[2], bytes[2]);
        assert_eq!(raw[3], bytes[3]);
        assert_eq!(raw[4], bytes[4]);
        assert_eq!(raw[5], bytes[5]);
        assert_eq!(raw[6], bytes[6]);
        assert_eq!(raw[7], bytes[7]);

        println!("Passed: {:?}", instruction)
    }
}