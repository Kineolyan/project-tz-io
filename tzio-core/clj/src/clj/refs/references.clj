(ns clj.refs.references
  (:import 
    (com.kineolyan.tzio.v1.api.ref 
      ValueReference
      SlotReference)))

(defn acc-slot
  [_] 
  [:acc])
(def to-acc-slot acc-slot)

(defn nil-slot
  ([_] [:nil]))
(def to-nil-slot nil-slot)

(defn value-slot
  [value]
  [:value value])
(defn to-value-slot
  [^ValueReference type] 
  (value-slot (.-value type)))

(defn ref-slot
  [ref]
  [:slot ref])
(defn to-ref-slot
  [^SlotReference type]
  (ref-slot (.-value type)))

(defn convert-input
  [type]
  (acc-slot))
