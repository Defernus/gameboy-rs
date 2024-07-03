#![allow(non_camel_case_types)]

use enum_dispatch::enum_dispatch;

use crate::*;

#[enum_dispatch]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Instruction {
    CCF(InstructionCCF),
    DI(InstructionDI),
    EI(InstructionEI),
    NOP(InstructionNOP),
    RLA(InstructionRLA),
    RRA(InstructionRRA),
    RRCA(InstructionRRCA),
    RST(InstructionRST),
    SCF(InstructionSCF),
    STOP(InstructionSTOP),
    CPL(InstructionCPL),
    DAA(InstructionDAA),
    RETI(InstructionRETI),
    RLCA(InstructionRLCA),
    HALT(InstructionHALT),
    ADC(InstructionADC),
    ADD(InstructionADD),
    AND(InstructionAND),
    BIT(InstructionBIT),
    CALL(InstructionCALL),
    CP(InstructionCP),
    DEC(InstructionDEC),
    INC(InstructionINC),
    JP(InstructionJP),
    JR(InstructionJR),
    LD(InstructionLD),
    LDH(InstructionLDH),
    OR(InstructionOR),
    POP(InstructionPOP),
    PUSH(InstructionPUSH),
    RES(InstructionRES),
    RET(InstructionRET),
    RL(InstructionRL),
    RLC(InstructionRLC),
    RR(InstructionRR),
    RRC(InstructionRRC),
    SBC(InstructionSBC),
    SET(InstructionSET),
    SLA(InstructionSLA),
    SRA(InstructionSRA),
    SRL(InstructionSRL),
    SUB(InstructionSUB),
    SWAP(InstructionSWAP),
    XOR(InstructionXOR),
}

#[enum_dispatch(Instruction)]
pub trait InstructionTrait {
    /// Execute the instruction and return the number of cycles it took.
    ///
    /// Returns 0 if the instruction is impossible.
    fn execute(&self, emulator: &mut Emulator) -> u8;
}

macro_rules! bit {
    (0) => {
        false
    };
    (1) => {
        true
    };
    ($bit:tt) => {
        $bit
    };
}

macro_rules! bits {
    ($($bit:tt),*) => {
        [$(bit!($bit)),*]
    };
}

impl Instruction {
    /// Read instruction from instruction register and memory.
    ///
    /// Returns the instruction and the number of bytes read.
    pub fn read(
        instruction_register: u8,
        memory: &Memory,
        program_counter: &mut ProgramCounter,
    ) -> Option<Self> {
        let opcode = instruction_register;

        let bits = to_bits(opcode);

        match bits {
            // ======================== Block 0 ========================

            // nop	0	0	0	0	0	0	0	0
            bits![0, 0, 0, 0, 0, 0, 0, 0] => Some(Self::NOP(InstructionNOP)),
            // ld r16, n16	0	0	Dest (r16)	0	0	0	1
            bits![0, 0, b0, b1, 0, 0, 0, 1] => {
                let r16 = ArgumentR16::from_bits(b0, b1);
                let n16 = memory.read_u16_at_pc(program_counter);

                Some(Self::LD(InstructionLD::R16_N16(r16, ArgumentN16(n16))))
            }
            // ld [r16mem], a	0	0	Dest (r16mem)	0	0	1	0
            bits![0, 0, b0, b1, 0, 0, 1, 0] => {
                let r16 = ArgumentR16::from_bits(b0, b1);

                let data = match r16 {
                    ArgumentR16::BC | ArgumentR16::DE => InstructionLD::AtR16_A(r16),
                    ArgumentR16::HL => InstructionLD::AtHLI_A,
                    ArgumentR16::SP => InstructionLD::AtHLD_A,
                };

                Some(Self::LD(data))
            }
            // ld a, [r16mem]	0	0	Source (r16mem)	1	0	1	0
            bits![0, 0, b0, b1, 1, 0, 1, 0] => {
                let r16 = ArgumentR16::from_bits(b0, b1);

                let data = match r16 {
                    ArgumentR16::BC | ArgumentR16::DE => InstructionLD::A_AtR16(r16),
                    ArgumentR16::HL => InstructionLD::A_AtHLI,
                    ArgumentR16::SP => InstructionLD::A_AtHLD,
                };

                Some(Self::LD(data))
            }
            // ld [n16], sp	0	0	0	0	1	0	0	0
            bits![0, 0, 0, 0, 1, 0, 0, 0] => {
                let n16 = memory.read_u16_at_pc(program_counter);

                Some(Self::LD(InstructionLD::AtN16_SP(ArgumentN16(n16))))
            }

            // inc r16	0	0	Operand (r16)	0	0	1	1
            bits![0, 0, b0, b1, 0, 0, 1, 1] => {
                let r16 = ArgumentR16::from_bits(b0, b1);

                Some(Self::INC(InstructionINC::R16(r16)))
            }
            // dec r16	0	0	Operand (r16)	1	0	1	1
            bits![0, 0, b0, b1, 1, 0, 1, 1] => {
                let r16 = ArgumentR16::from_bits(b0, b1);

                Some(Self::DEC(InstructionDEC::R16(r16)))
            }
            // add hl, r16	0	0	Operand (r16)	1	0	0	1
            bits![0, 0, b0, b1, 1, 0, 0, 1] => {
                let r16 = ArgumentR16::from_bits(b0, b1);

                Some(Self::ADD(InstructionADD::HL_R16(r16)))
            }

            // inc r8	0	0	Operand (r8)	1	0	0
            bits![0, 0, b0, b1, b2, 1, 0, 0] => {
                let r8 = ArgumentR8::from_bits(b0, b1, b2);

                Some(Self::INC(InstructionINC::R8(r8)))
            }
            // dec r8	0	0	Operand (r8)	1	0	1
            bits![0, 0, b0, b1, b2, 1, 0, 1] => {
                let r8 = ArgumentR8::from_bits(b0, b1, b2);

                Some(Self::DEC(InstructionDEC::R8(r8)))
            }

            // ld r8, n8	0	0	Dest (r8)	1	1	0
            bits![0, 0, b0, b1, b3, 1, 1, 0] => {
                let r8 = ArgumentR8::from_bits(b0, b1, b3);

                let n8 = memory.read_u8_at_pc(program_counter);

                Some(Self::LD(InstructionLD::R8_N8(r8, ArgumentN8(n8))))
            }

            // rlca	0	0	0	0	0	1	1	1
            bits![0, 0, 0, 0, 0, 1, 1, 1] => Some(Self::RLCA(InstructionRLCA)),
            // rrca	0	0	0	0	1	1	1	1
            bits![0, 0, 0, 0, 1, 1, 1, 1] => Some(Self::RRCA(InstructionRRCA)),
            // rla	0	0	0	1	0	1	1	1
            bits![0, 0, 0, 1, 0, 1, 1, 1] => Some(Self::RLA(InstructionRLA)),
            // rra	0	0	0	1	1	1	1	1
            bits![0, 0, 0, 1, 1, 1, 1, 1] => Some(Self::RRA(InstructionRRA)),
            // daa	0	0	1	0	0	1	1	1
            bits![0, 0, 1, 0, 0, 1, 1, 1] => Some(Self::DAA(InstructionDAA)),
            // cpl	0	0	1	0	1	1	1	1
            bits![0, 0, 1, 0, 1, 1, 1, 1] => Some(Self::CPL(InstructionCPL)),
            // scf	0	0	1	1	0	1	1	1
            bits![0, 0, 1, 1, 0, 1, 1, 1] => Some(Self::SCF(InstructionSCF)),
            // ccf	0	0	1	1	1	1	1	1
            bits![0, 0, 1, 1, 1, 1, 1, 1] => Some(Self::CCF(InstructionCCF)),

            // jr e8	0	0	0	1	1	0	0	0
            bits![0, 0, 0, 1, 1, 0, 0, 0] => {
                let e8 = memory.read_i8_at_pc(program_counter);

                Some(Self::JR(InstructionJR::E8(ArgumentE8(e8))))
            }
            // jr cond, e8	0	0	1	Condition (cond)	0	0	0
            bits![0, 0, 1, b0, b1, 0, 0, 0] => {
                let cond = ArgumentCC::from_bits(b0, b1);
                let e8 = memory.read_i8_at_pc(program_counter);

                Some(Self::JR(InstructionJR::CC_E8(cond, ArgumentE8(e8))))
            }

            // stop	0	0	0	1	0	0	0	0
            bits![0, 0, 0, 1, 0, 0, 0, 0] => Some(Self::STOP(InstructionSTOP)),

            // ======= Block 1: 8-bit register-to-register loads =======

            // halt	0	1	1	1	0	1	1	0
            bits![0, 1, 1, 1, 0, 1, 1, 0] => Some(Self::HALT(InstructionHALT)),

            // ld r8, r8	0	1	Dest (r8)	Source (r8)
            bits![0, 1, b0, b1, b2, b3, b4, b5] => {
                let dest = ArgumentR8::from_bits(b0, b1, b2);
                let source = ArgumentR8::from_bits(b3, b4, b5);

                Some(Self::LD(InstructionLD::R8_R8(dest, source)))
            }

            // =============== Block 2: 8-bit arithmetic ===============

            // add a, r8	1	0	0	0	0	Operand (r8)
            bits![1, 0, 0, 0, 0, b0, b1, b2] => {
                let r8 = ArgumentR8::from_bits(b0, b1, b2);

                Some(Self::ADD(InstructionADD::A_R8(r8)))
            }
            // adc a, r8	1	0	0	0	1	Operand (r8)
            bits![1, 0, 0, 0, 1, b0, b1, b2] => {
                let r8 = ArgumentR8::from_bits(b0, b1, b2);

                Some(Self::ADC(InstructionADC::A_R8(r8)))
            }
            // sub a, r8	1	0	0	1	0	Operand (r8)
            bits![1, 0, 0, 1, 0, b0, b1, b2] => {
                let r8 = ArgumentR8::from_bits(b0, b1, b2);

                Some(Self::SUB(InstructionSUB::A_R8(r8)))
            }
            // sbc a, r8	1	0	0	1	1	Operand (r8)
            bits![1, 0, 0, 1, 1, b0, b1, b2] => {
                let r8 = ArgumentR8::from_bits(b0, b1, b2);

                Some(Self::SBC(InstructionSBC::A_R8(r8)))
            }
            // and a, r8	1	0	1	0	0	Operand (r8)
            bits![1, 0, 1, 0, 0, b0, b1, b2] => {
                let r8 = ArgumentR8::from_bits(b0, b1, b2);

                Some(Self::AND(InstructionAND::A_R8(r8)))
            }
            // xor a, r8	1	0	1	0	1	Operand (r8)
            bits![1, 0, 1, 0, 1, b0, b1, b2] => {
                let r8 = ArgumentR8::from_bits(b0, b1, b2);

                Some(Self::XOR(InstructionXOR::A_R8(r8)))
            }
            // or a, r8	1	0	1	1	0	Operand (r8)
            bits![1, 0, 1, 1, 0, b0, b1, b2] => {
                let r8 = ArgumentR8::from_bits(b0, b1, b2);

                Some(Self::OR(InstructionOR::A_R8(r8)))
            }
            // cp a, r8	1	0	1	1	1	Operand (r8)
            bits![1, 0, 1, 1, 1, b0, b1, b2] => {
                let r8 = ArgumentR8::from_bits(b0, b1, b2);

                Some(Self::CP(InstructionCP::A_R8(r8)))
            }

            // ======================== Block 3 ========================

            // add a, n8	1	1	0	0	0	1	1	0
            bits![1, 1, 0, 0, 0, 1, 1, 0] => {
                let n8 = memory.read_u8_at_pc(program_counter);

                Some(Self::ADD(InstructionADD::A_N8(ArgumentN8(n8))))
            }
            // adc a, n8	1	1	0	0	1	1	1	0
            bits![1, 1, 0, 0, 1, 1, 1, 0] => {
                let n8 = memory.read_u8_at_pc(program_counter);

                Some(Self::ADC(InstructionADC::A_N8(ArgumentN8(n8))))
            }
            // sub a, n8	1	1	0	1	0	1	1	0
            bits![1, 1, 0, 1, 0, 1, 1, 0] => {
                let n8 = memory.read_u8_at_pc(program_counter);

                Some(Self::SUB(InstructionSUB::A_N8(ArgumentN8(n8))))
            }
            // sbc a, n8	1	1	0	1	1	1	1	0
            bits![1, 1, 0, 1, 1, 1, 1, 0] => {
                let n8 = memory.read_u8_at_pc(program_counter);

                Some(Self::SBC(InstructionSBC::A_N8(ArgumentN8(n8))))
            }
            // and a, n8	1	1	1	0	0	1	1	0
            bits![1, 1, 1, 0, 0, 1, 1, 0] => {
                let n8 = memory.read_u8_at_pc(program_counter);

                Some(Self::AND(InstructionAND::A_N8(ArgumentN8(n8))))
            }
            // xor a, n8	1	1	1	0	1	1	1	0
            bits![1, 1, 1, 0, 1, 1, 1, 0] => {
                let n8 = memory.read_u8_at_pc(program_counter);

                Some(Self::XOR(InstructionXOR::A_N8(ArgumentN8(n8))))
            }
            // or a, n8	1	1	1	1	0	1	1	0
            bits![1, 1, 1, 1, 0, 1, 1, 0] => {
                let n8 = memory.read_u8_at_pc(program_counter);

                Some(Self::OR(InstructionOR::A_N8(ArgumentN8(n8))))
            }
            // cp a, n8	1	1	1	1	1	1	1	0
            bits![1, 1, 1, 1, 1, 1, 1, 0] => {
                let n8 = memory.read_u8_at_pc(program_counter);

                Some(Self::CP(InstructionCP::A_N8(ArgumentN8(n8))))
            }

            // ret cond	1	1	0	Condition (cond)	0	0	0
            bits![1, 1, 0, b0, b1, 0, 0, 0] => {
                let cond = ArgumentCC::from_bits(b0, b1);

                Some(Self::RET(InstructionRET(Some(cond))))
            }
            // ret	1	1	0	0	1	0	0	1
            bits![1, 1, 0, 0, 1, 0, 0, 1] => Some(Self::RET(InstructionRET(None))),
            // reti	1	1	0	1	1	0	0	1
            bits![1, 1, 0, 1, 1, 0, 0, 1] => Some(Self::RETI(InstructionRETI)),
            // jp cond, n16	1	1	0	Condition (cond)	0	1	0
            bits![1, 1, 0, b0, b1, 0, 1, 0] => {
                let cond = ArgumentCC::from_bits(b0, b1);
                let n16 = memory.read_u16_at_pc(program_counter);

                Some(Self::JP(InstructionJP::CC_N16(cond, ArgumentN16(n16))))
            }
            // jp n16	1	1	0	0	0	0	1	1
            bits![1, 1, 0, 0, 0, 0, 1, 1] => {
                let n16 = memory.read_u16_at_pc(program_counter);

                Some(Self::JP(InstructionJP::N16(ArgumentN16(n16))))
            }
            // jp hl	1	1	1	0	1	0	0	1
            bits![1, 1, 1, 0, 1, 0, 0, 1] => Some(Self::JP(InstructionJP::HL)),
            // call cond, n16	1	1	0	Condition (cond)	1	0	0
            bits![1, 1, 0, b0, b1, 1, 0, 0] => {
                let cond = ArgumentCC::from_bits(b0, b1);
                let n16 = memory.read_u16_at_pc(program_counter);

                Some(Self::CALL(InstructionCALL::CC_N16(cond, ArgumentN16(n16))))
            }
            // call n16	1	1	0	0	1	1	0	1
            bits![1, 1, 0, 0, 1, 1, 0, 1] => {
                let n16 = memory.read_u16_at_pc(program_counter);

                Some(Self::CALL(InstructionCALL::N16(ArgumentN16(n16))))
            }
            // rst tgt3	1	1	Target (tgt3)	1	1	1
            bits![1, 1, b0, b1, b2, 1, 1, 1] => {
                let vec = ArgumentVec::from_bits(b0, b1, b2);

                Some(Self::RST(InstructionRST(vec)))
            }

            // pop r16stk	1	1	Register (r16stk)	0	0	0	1
            bits![1, 1, b0, b1, 0, 0, 0, 1] => {
                let r16_stk = ArgumentStkR16::from_bits(b0, b1);

                Some(Self::POP(InstructionPOP(r16_stk)))
            }
            // push r16stk	1	1	Register (r16stk)	0	1	0	1
            bits![1, 1, b0, b1, 0, 1, 0, 1] => {
                let r16_stk = ArgumentStkR16::from_bits(b0, b1);

                Some(Self::PUSH(InstructionPUSH(r16_stk)))
            }

            // ldh [c], a	1	1	1	0	0	0	1	0
            bits![1, 1, 1, 0, 0, 0, 1, 0] => Some(Self::LDH(InstructionLDH::AtC_A)),
            // ldh [n8], a	1	1	1	0	0	0	0	0
            bits![1, 1, 1, 0, 0, 0, 0, 0] => {
                let n8 = memory.read_u8_at_pc(program_counter);

                Some(Self::LDH(InstructionLDH::AtN8_A(ArgumentN8(n8))))
            }
            // ld [n16], a	1	1	1	0	1	0	1	0
            bits![1, 1, 1, 0, 1, 0, 1, 0] => {
                let n16 = memory.read_u16_at_pc(program_counter);

                Some(Self::LD(InstructionLD::AtN16_A(ArgumentN16(n16))))
            }
            // ldh a, [c]	1	1	1	1	0	0	1	0
            bits![1, 1, 1, 1, 0, 0, 1, 0] => Some(Self::LDH(InstructionLDH::A_AtC)),
            // ldh a, [n8]	1	1	1	1	0	0	0	0
            bits![1, 1, 1, 1, 0, 0, 0, 0] => {
                let n8 = memory.read_u8_at_pc(program_counter);

                Some(Self::LDH(InstructionLDH::A_AtN8(ArgumentN8(n8))))
            }
            // ld a, [n16]	1	1	1	1	1	0	1	0
            bits![1, 1, 1, 1, 1, 0, 1, 0] => {
                let n16 = memory.read_u16_at_pc(program_counter);

                Some(Self::LD(InstructionLD::A_AtN16(ArgumentN16(n16))))
            }

            // add sp, e8	1	1	1	0	1	0	0	0
            bits![1, 1, 1, 0, 1, 0, 0, 0] => {
                let e8 = memory.read_i8_at_pc(program_counter);

                Some(Self::ADD(InstructionADD::SP_E8(ArgumentE8(e8))))
            }
            // ld hl, sp + e8	1	1	1	1	1	0	0	0
            bits![1, 1, 1, 1, 1, 0, 0, 0] => {
                let e8 = memory.read_i8_at_pc(program_counter);

                Some(Self::LD(InstructionLD::HL_SP_E8(ArgumentE8(e8))))
            }
            // ld sp, hl	1	1	1	1	1	0	0	1
            bits![1, 1, 1, 1, 1, 0, 0, 1] => Some(Self::LD(InstructionLD::SP_HL)),

            // di	1	1	1	1	0	0	1	1
            bits![1, 1, 1, 1, 0, 0, 1, 1] => Some(Self::DI(InstructionDI)),
            // ei	1	1	1	1	1	0	1	1
            bits![1, 1, 1, 1, 1, 0, 1, 1] => Some(Self::EI(InstructionEI)),

            // Prefix (see block below)	1	1	0	0	1	0	1	1
            bits![1, 1, 0, 0, 1, 0, 1, 1] => {
                let opcode = memory.read_u8_at_pc(program_counter);
                let bits = to_bits(opcode);

                match bits {
                    // rlc r8	0	0	0	0	0	Operand (r8)
                    bits![0, 0, 0, 0, 0, b0, b1, b2] => {
                        let r8 = ArgumentR8::from_bits(b0, b1, b2);

                        Some(Self::RLC(InstructionRLC(r8)))
                    }
                    // rrc r8	0	0	0	0	1	Operand (r8)
                    bits![0, 0, 0, 0, 1, b0, b1, b2] => {
                        let r8 = ArgumentR8::from_bits(b0, b1, b2);

                        Some(Self::RRC(InstructionRRC(r8)))
                    }
                    // rl r8	0	0	0	1	0	Operand (r8)
                    bits![0, 0, 0, 1, 0, b0, b1, b2] => {
                        let r8 = ArgumentR8::from_bits(b0, b1, b2);

                        Some(Self::RL(InstructionRL(r8)))
                    }
                    // rr r8	0	0	0	1	1	Operand (r8)
                    bits![0, 0, 0, 1, 1, b0, b1, b2] => {
                        let r8 = ArgumentR8::from_bits(b0, b1, b2);

                        Some(Self::RR(InstructionRR(r8)))
                    }
                    // sla r8	0	0	1	0	0	Operand (r8)
                    bits![0, 0, 1, 0, 0, b0, b1, b2] => {
                        let r8 = ArgumentR8::from_bits(b0, b1, b2);

                        Some(Self::SLA(InstructionSLA(r8)))
                    }
                    // sra r8	0	0	1	0	1	Operand (r8)
                    bits![0, 0, 1, 0, 1, b0, b1, b2] => {
                        let r8 = ArgumentR8::from_bits(b0, b1, b2);

                        Some(Self::SRA(InstructionSRA(r8)))
                    }
                    // swap r8	0	0	1	1	0	Operand (r8)
                    bits![0, 0, 1, 1, 0, b0, b1, b2] => {
                        let r8 = ArgumentR8::from_bits(b0, b1, b2);

                        Some(Self::SWAP(InstructionSWAP(r8)))
                    }
                    // srl r8	0	0	1	1	1	Operand (r8)
                    bits![0, 0, 1, 1, 1, b0, b1, b2] => {
                        let r8 = ArgumentR8::from_bits(b0, b1, b2);

                        Some(Self::SRL(InstructionSRL(r8)))
                    }

                    // bit u3, r8	0	1	Bit index (u3)	Operand (r8)
                    bits![0, 1, b0, b1, b2, b3, b4, b5] => {
                        let u3 = ArgumentU3::from_bits(b0, b1, b2);
                        let r8 = ArgumentR8::from_bits(b3, b4, b5);

                        Some(Self::BIT(InstructionBIT(u3, r8)))
                    }
                    // res u3, r8	1	0	Bit index (u3)	Operand (r8)
                    bits![1, 0, b0, b1, b2, b3, b4, b5] => {
                        let u3 = ArgumentU3::from_bits(b0, b1, b2);
                        let r8 = ArgumentR8::from_bits(b3, b4, b5);

                        Some(Self::RES(InstructionRES(u3, r8)))
                    }
                    // set u3, r8	1	1	Bit index (u3)	Operand (r8)
                    bits![1, 1, b0, b1, b2, b3, b4, b5] => {
                        let u3 = ArgumentU3::from_bits(b0, b1, b2);
                        let r8 = ArgumentR8::from_bits(b3, b4, b5);

                        Some(Self::SET(InstructionSET(u3, r8)))
                    }
                }
            }

            _ => None,
        }
    }
}

pub enum InstructionReadError {
    /// The opcode is not recognized.
    UnknownOpcode(u8),
}

pub fn to_bits(value: u8) -> [bool; 8] {
    [
        value & (1 << 7) != 0,
        value & (1 << 6) != 0,
        value & (1 << 5) != 0,
        value & (1 << 4) != 0,
        value & (1 << 3) != 0,
        value & (1 << 2) != 0,
        value & (1 << 1) != 0,
        value & 1 != 0,
    ]
}

#[test]
fn test_to_bits() {
    assert_eq!(to_bits(0b0000_0000), [false; 8]);
    assert_eq!(to_bits(0b1111_1111), [true; 8]);
    assert_eq!(
        to_bits(0b1010_1010),
        [true, false, true, false, true, false, true, false]
    );
}
