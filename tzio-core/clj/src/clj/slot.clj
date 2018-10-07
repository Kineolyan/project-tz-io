(ns clj.slot)

(defn empty-slot
  "Creates an empty slot ready to receive new data"
  [] 
  [:empty])

(defn data-slot
  "Creates a slot with an int datum inside"
  [value] 
  [:slot value])
  
(defn queue-slot
  "Creates a slot queuing the input values"
  [& rest] 
  (cons :queue (into [] rest)))

(defn get-type [[type & remaining] & _] type)

(defmulti is-queue get-type)
(defmethod is-queue :queue [_] true)
(defmethod is-queue :default [_] false)

(defmulti can-read get-type)
(defmethod can-read :slot [_] true)
(defmethod can-read :empty [_] false)
(defmethod can-read :queue 
  [[_ & vals]] 
  (not (empty? vals)))

(defmulti can-write get-type)
(defmethod can-write :slot [_] false)
(defmethod can-write :empty [_] true)
(defmethod can-write :queue [_] true)

(defmulti read-slot get-type)
(defmethod read-slot :slot 
  [[_ value]] 
  [
    value 
    (empty-slot)])
(defmethod read-slot :queue
  [[_ value & rest]]
  [
    value
    (queue-slot rest)])

(defmulti write-slot get-type)
(defmethod write-slot :empty
  [_ value]
  (data-slot value))
(defmethod write-slot :queue
  [[_ & values] value]
  (apply queue-slot (conj (into [] values) value)))
