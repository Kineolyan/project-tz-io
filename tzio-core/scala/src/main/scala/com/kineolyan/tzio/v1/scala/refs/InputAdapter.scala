package com.kineolyan.tzio.v1.scala.refs

import com.kineolyan.tzio.v1.api.ref

object InputAdapter extends ref.InputReferenceVisitor[InputReference] {

  def convert(definition: ref.InputReferenceType): InputReference =
    definition.accept(this)

  override def visit(ref: ref.SlotReference): InputReference =
    InSlotReference(ref.slot)

  override def visit(ref: ref.AccReference): InputReference =
    InAccReference()

  override def visit(ref: ref.ValueReference): InputReference =
    ValueReference(ref.value)

  override def visit(ref: ref.NilReference): InputReference =
    InNilReference()
}
