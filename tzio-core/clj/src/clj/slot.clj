(ns clj.slot)

(defn empty-slot
  "Creates an empty slot ready to receive new data"
  ([] [:empty]))

(defn data-slot
  "Creates a slot with an int datum inside"
  ([value] [:slot value]))
  
(defn queue-slot
  "Creates a slot queuing the input values"
  ([] [:queue []]))

(defmulti is-queue (fn [[slot & remaining]] slot))
(defmethod is-queue :queue [& _] true)
(defmethod is-queue :default [& _] false)

(defn enqueue
  "Enqueues a value into a queue"
  [[type values] value]
  [type (conj values value)])

(defn dequeue
  "Dequeues the first value from a queue"
  [[type [value & rest]]]
  [
    [type rest]
    value])

