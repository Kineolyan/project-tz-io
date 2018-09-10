(ns clj.slot)

(defn empty-slot
  "Creates an empty slot ready to receive new data"
  []
  (:empty))

(defn data-slot
  "Creates a slot with an int datum inside"
  [value]
  (:slot value))
  
(defn queue-slot
  "Creates a slot queuing the input values"
  []
  (:queue []))

(defmulti is-queue (fn [[slot & remaining]] slot))
(defmethod is-queue [:queue] [_] true)
(defmethod is-queue [:slot] [_] false)
(defmethod is-queue [:empty] [_] false)
