package com.kineolyan.tzio.v1.scala.refs

import com.kineolyan.tzio.v1.api.ref

object OutputAdapter extends ref.OutputReferenceVisitor[OutputReference] {

  def convert(definition: ref.OutputReferenceType): OutputReference =
    definition.accept(this)

  override def visit(ref: ref.SlotReference): OutputReference =
    OutSlotReference(ref.slot)

  override def visit(ref: ref.AccReference): OutputReference =
    OutAccReference()

  override def visit(ref: ref.NilReference): OutputReference =
    OutNilReference()
}
