//! This module defines common traits and characteristics from all three
//! x86 processor modes.
use core::fmt::{Debug, Display};
use yaxpeax_arch::{
    AddressDiff, Arch, DecodeErrorBounds, Decoder, Instruction, LengthedInstruction, Reader,
};

use crate::{x86_16, x86_32, x86_64};
use crate::{Opcode, Operand, Segment};

/// A pseudo-architecture that further defines one of x86's three processor modes.
pub trait X86Arch: Arch {
    /// The specific type of the mode's instruction.
    type Instruction: X86Instruction
        + LengthedInstruction<Unit = AddressDiff<u64>>
        + Debug
        + Default
        + Sized;
    type Decoder: X86Decoder<Self> + Default;
}

impl X86Arch for x86_16 {
    type Instruction = crate::real_mode::Instruction;
    type Decoder = crate::real_mode::InstDecoder;
}

impl X86Arch for x86_32 {
    type Instruction = crate::protected_mode::Instruction;
    type Decoder = crate::protected_mode::InstDecoder;
}

impl X86Arch for x86_64 {
    type Instruction = crate::long_mode::Instruction;
    type Decoder = crate::long_mode::InstDecoder;
}

pub trait X86Decoder<A: X86Arch + ?Sized> {
    /// decode one instruction for this architecture from the [`crate::Reader`] of this
    /// architecture's `Word`.
    fn decode<T: Reader<u64, u8>>(
        &self,
        words: &mut T,
    ) -> Result<<A as X86Arch>::Instruction, A::DecodeError> {
        let mut inst = <A as X86Arch>::Instruction::default();
        self.decode_into(&mut inst, words).map(|_: ()| inst)
    }

    /// decode one instruction for this architecture from the [`crate::Reader`] of this
    /// architecture's `Word`, writing into the provided `inst`.
    ///
    /// SAFETY:
    ///
    /// while `inst` MUST be left in a state that does not violate Rust's safety guarantees,
    /// implementors are NOT obligated to leave `inst` in a semantically meaningful state if
    /// decoding fails. if `decode_into` returns an error, callers may find contradictory and
    /// useless information in `inst`, as well as *stale data* from whatever was passed in.
    fn decode_into<T: Reader<u64, u8>>(
        &self,
        inst: &mut <A as X86Arch>::Instruction,
        words: &mut T,
    ) -> Result<(), A::DecodeError>;
}

impl X86Decoder<x86_16> for crate::real_mode::InstDecoder {
    fn decode_into<T: Reader<u64, u8>>(
        &self,
        inst: &mut <x86_16 as X86Arch>::Instruction,
        words: &mut T,
    ) -> Result<(), <x86_16 as Arch>::DecodeError> {
        Decoder::decode_into(self, inst, words)
    }
}

impl X86Decoder<x86_32> for crate::protected_mode::InstDecoder {
    fn decode_into<T: Reader<u64, u8>>(
        &self,
        inst: &mut <x86_32 as X86Arch>::Instruction,
        words: &mut T,
    ) -> Result<(), <x86_32 as Arch>::DecodeError> {
        Decoder::decode_into(self, inst, words)
    }
}

impl X86Decoder<x86_64> for crate::long_mode::InstDecoder {
    fn decode_into<T: Reader<u64, u8>>(
        &self,
        inst: &mut <x86_64 as X86Arch>::Instruction,
        words: &mut T,
    ) -> Result<(), <x86_64 as Arch>::DecodeError> {
        Decoder::decode_into(self, inst, words)
    }
}

pub trait X86Instruction: Instruction + core::fmt::Display {
    /// Get the opcode of this instruction
    fn opcode(&self) -> Opcode;

    /// get the `Operand` at the provided index.
    ///
    /// panics if the index is `>= 4`.
    fn operand(&self, i: u8) -> Operand;

    /// get the number of operands in this instruction. useful in iterating an instruction's
    /// operands generically.
    fn operand_count(&self) -> u8;

    /// get the `Segment` that will *actually* be used for accessing the operand at index `i`.
    ///
    /// `stos`, `lods`, `movs`, and `cmps` specifically name some segments for use regardless of
    /// prefixes.
    fn segment_override_for_op(&self, op: u8) -> Option<Segment>;
}

impl X86Instruction for crate::long_mode::Instruction {
    fn opcode(&self) -> Opcode {
        self.opcode()
    }

    fn operand(&self, i: u8) -> Operand {
        self.operand(i)
    }

    fn operand_count(&self) -> u8 {
        self.operand_count()
    }

    fn segment_override_for_op(&self, op: u8) -> Option<Segment> {
        self.segment_override_for_op(op)
    }
}

impl X86Instruction for crate::protected_mode::Instruction {
    fn opcode(&self) -> Opcode {
        self.opcode()
    }

    fn operand(&self, i: u8) -> Operand {
        self.operand(i)
    }

    fn operand_count(&self) -> u8 {
        self.operand_count()
    }

    fn segment_override_for_op(&self, op: u8) -> Option<Segment> {
        self.segment_override_for_op(op)
    }
}

impl X86Instruction for crate::real_mode::Instruction {
    fn opcode(&self) -> Opcode {
        self.opcode()
    }

    fn operand(&self, i: u8) -> Operand {
        self.operand(i)
    }

    fn operand_count(&self) -> u8 {
        self.operand_count()
    }

    fn segment_override_for_op(&self, op: u8) -> Option<Segment> {
        self.segment_override_for_op(op)
    }
}
