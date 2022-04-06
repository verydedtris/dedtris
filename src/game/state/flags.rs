pub type FlagStore = u8;

pub enum Flag
{
	PieceHoldEnabled = 0x1,
}

pub fn switch(flags: &mut FlagStore, flag: Flag, state: bool)
{
	*flags = *flags | (flag as FlagStore * state as FlagStore);
}

pub fn check(flags: &FlagStore, flag: Flag) -> bool
{
	(*flags & flag as FlagStore) != 0
}
