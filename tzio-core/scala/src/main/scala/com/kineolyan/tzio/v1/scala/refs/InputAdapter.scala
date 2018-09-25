package com.kineolyan.tzio.v1.scala.refs

import com.kineolyan.tzio.v1.api.ref

/**
  * Object adapting definition of inputs to their actual types in the Scala core
  */
object InputAdapter extends ref.InputReferenceVisitor[InputReference] {

  def convert(definition: ref.InputReferenceType): InputReference =
    definition.accept(this)

  override def visit(reference: ref.SlotReference): InputReference =
    InSlotReference(reference.slot)

  override def visit(reference: ref.AccReference): InputReference =
    InAccReference()

  override def visit(reference: ref.ValueReference): InputReference =
    ValueReference(reference.value)

  override def visit(reference: ref.NilReference): InputReference =
    InNilReference()
}
