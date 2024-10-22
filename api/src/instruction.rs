use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum MyProjectInstruction {
    Initialize = 0,
    Add = 1,
    TransferSol = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Add {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TransferSol {
    pub amount: [u8; 8],
}

instruction!(MyProjectInstruction, Initialize);
instruction!(MyProjectInstruction, Add);
instruction!(MyProjectInstruction, TransferSol);
