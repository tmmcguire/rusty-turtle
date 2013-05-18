// generated by TurtleScript write-rust-ops.js

enum Op {
  Op_push_frame = 0,
  Op_push_literal = 1,
  Op_new_object = 2,
  Op_new_array = 3,
  Op_new_function = 4,
  Op_get_slot_direct = 5,
  Op_get_slot_indirect = 6,
  Op_get_slot_direct_check = 7,
  Op_set_slot_direct = 8,
  Op_set_slot_indirect = 9,
  Op_invoke = 10,
  Op_return = 11,
  Op_jmp = 12,
  Op_jmp_unless = 13,
  Op_pop = 14,
  Op_dup = 15,
  Op_2dup = 16,
  Op_over = 17,
  Op_over2 = 18,
  Op_swap = 19,
  Op_un_not = 20,
  Op_un_minus = 21,
  Op_un_typeof = 22,
  Op_bi_eq = 23,
  Op_bi_gt = 24,
  Op_bi_gte = 25,
  Op_bi_add = 26,
  Op_bi_sub = 27,
  Op_bi_mul = 28,
  Op_bi_div = 29
}

impl Op {
  fn args(&self) -> uint {
    match *self {
      Op_push_frame => 0,
      Op_push_literal => 1,
      Op_new_object => 0,
      Op_new_array => 0,
      Op_new_function => 1,
      Op_get_slot_direct => 1,
      Op_get_slot_indirect => 0,
      Op_get_slot_direct_check => 1,
      Op_set_slot_direct => 1,
      Op_set_slot_indirect => 0,
      Op_invoke => 1,
      Op_return => 0,
      Op_jmp => 1,
      Op_jmp_unless => 1,
      Op_pop => 0,
      Op_dup => 0,
      Op_2dup => 0,
      Op_over => 0,
      Op_over2 => 0,
      Op_swap => 0,
      Op_un_not => 0,
      Op_un_minus => 0,
      Op_un_typeof => 0,
      Op_bi_eq => 0,
      Op_bi_gt => 0,
      Op_bi_gte => 0,
      Op_bi_add => 0,
      Op_bi_sub => 0,
      Op_bi_mul => 0,
      Op_bi_div => 0
    }
  }
  fn stackpush(&self) -> uint {
    match *self {
      Op_push_frame => 1,
      Op_push_literal => 1,
      Op_new_object => 1,
      Op_new_array => 1,
      Op_new_function => 1,
      Op_get_slot_direct => 1,
      Op_get_slot_indirect => 1,
      Op_get_slot_direct_check => 1,
      Op_set_slot_direct => 0,
      Op_set_slot_indirect => 0,
      Op_invoke => 1,
      Op_return => 0,
      Op_jmp => 0,
      Op_jmp_unless => 0,
      Op_pop => 0,
      Op_dup => 2,
      Op_2dup => 4,
      Op_over => 3,
      Op_over2 => 4,
      Op_swap => 2,
      Op_un_not => 1,
      Op_un_minus => 1,
      Op_un_typeof => 1,
      Op_bi_eq => 1,
      Op_bi_gt => 1,
      Op_bi_gte => 1,
      Op_bi_add => 1,
      Op_bi_sub => 1,
      Op_bi_mul => 1,
      Op_bi_div => 1
    }
  }
  fn stackpop(&self, args: &[int]) -> uint {
    match *self {
      Op_push_frame => 0,
      Op_push_literal => 0,
      Op_new_object => 0,
      Op_new_array => 0,
      Op_new_function => 0,
      Op_get_slot_direct => 1,
      Op_get_slot_indirect => 2,
      Op_get_slot_direct_check => 1,
      Op_set_slot_direct => 2,
      Op_set_slot_indirect => 3,
      Op_invoke => (args[0] as uint) + 2,
      Op_return => 1,
      Op_jmp => 0,
      Op_jmp_unless => 1,
      Op_pop => 1,
      Op_dup => 1,
      Op_2dup => 2,
      Op_over => 2,
      Op_over2 => 3,
      Op_swap => 2,
      Op_un_not => 1,
      Op_un_minus => 1,
      Op_un_typeof => 1,
      Op_bi_eq => 2,
      Op_bi_gt => 2,
      Op_bi_gte => 2,
      Op_bi_add => 2,
      Op_bi_sub => 2,
      Op_bi_mul => 2,
      Op_bi_div => 2
    }
  }
  fn new_from_int(val: int) -> Op {
    match val {
      0 => Op_push_frame,
      1 => Op_push_literal,
      2 => Op_new_object,
      3 => Op_new_array,
      4 => Op_new_function,
      5 => Op_get_slot_direct,
      6 => Op_get_slot_indirect,
      7 => Op_get_slot_direct_check,
      8 => Op_set_slot_direct,
      9 => Op_set_slot_indirect,
      10 => Op_invoke,
      11 => Op_return,
      12 => Op_jmp,
      13 => Op_jmp_unless,
      14 => Op_pop,
      15 => Op_dup,
      16 => Op_2dup,
      17 => Op_over,
      18 => Op_over2,
      19 => Op_swap,
      20 => Op_un_not,
      21 => Op_un_minus,
      22 => Op_un_typeof,
      23 => Op_bi_eq,
      24 => Op_bi_gt,
      25 => Op_bi_gte,
      26 => Op_bi_add,
      27 => Op_bi_sub,
      28 => Op_bi_mul,
      29 => Op_bi_div,
      _ => fail!()
    }
  }
}

#[test]
fn test_invoke() {
  let op = Op_invoke;
  let args : &[int] = &[3];
  assert!(op.stackpop(args) == 5);
}
#[test]
fn test_cast() {
  let op1a = Op_push_literal;
  let op1b = Op::new_from_int(1);
  assert!((op1a as int) == 1);
  assert!((op1b as int) == 1);
}
