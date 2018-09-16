package com.kineolyan.tzio.v1.scala.refs

import com.kineolyan.tzio.v1.api.ref

object OutputAdapter extends ref.OutputReferenceVisitor[OutputReference] {

  def convert(definition: ref.OutputReferenceType): OutputReference =
    definition.accept(this)

  override def visit(reference: ref.SlotReference): OutputReference =
    OutSlotReference(reference.slot)

  override def visit(reference: ref.AccReference): OutputReference =
    OutAccReference()

  override def visit(reference: ref.NilReference): OutputReference =
    OutNilReference()
}
