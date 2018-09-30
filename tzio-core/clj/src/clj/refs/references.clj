(ns clj.refs.references
  (:import 
    (com.kineolyan.tzio.v1.api.ref 
      ValueReference
      SlotReference)))

(defn acc-slot
  [_] 
  [:acc])

(defn nil-slot
  ([_] [:nil]))

(defn value-slot
  [^ValueReference type] 
  [:value (.-value type)])

(defn ref-slot
  [^SlotReference type]
  [:slot (.-value type)])

(defn convert
  [type]
  (acc-slot))
