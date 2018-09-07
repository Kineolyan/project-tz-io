(ns clj.slot)

(defn empty-slot
  "Creates an empty slot ready to receive new data"
  []
  :empty)

(defn data-slot
  "Creates a slot with an int datum inside"
  [value]
  (:slot value))
  
(defn queue-slot
  "Creates a slot queuing the input values"
  []
  [])
