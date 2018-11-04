(ns clj.ops.context
  (:require [clj.refs.references :as refs]
            [clj.slots :as sl]))

; Contexts are maps of :node, :inputs and :outputs
; for a given node of the TZ env

(defn slot-type 
  ([ctx [type & _]] type)
  ([ctx [type & _] val] type))
(def get-slot-type
  "Gets the type of the slot #idx of the context"
  "Type is the first value of a slot"
  [context type idx]
  (first (nth (type context) (dec idx))))

(defmulti can-read slot-type)
(defmethod can-read :slot
  [context [:slot idx]]
  (let [
        slot (nth (:inputs context) (dec idx))]
    (sl/can-read slot)))
(defmethod can-read :default
  [context ref]
  false)

(defmulti can-write slot-type)
(defmethod can-write :slot
  [{:keys [outputs]} context [:slot idx]]
  (let [
        slot (nth outputs (dec idx))]
    (sl/can-write slot)))
(defmethod can-write :acc [_ _] true)
(defmethod can-write :nil [_ _] true)

(defmulti read-value (fn [context idx] (get-slot-type context :inputs idx)))
(defmethod read-value :slot
  [context idx]
  (let [
        i (dec idx)
        inputs (:inputs context)
        slot (nth inputs i)
        [value new-slot] (sl/read-slot slot)
        new-inputs (assoc i new-slot inputs)
        new-context (assoc :inputs new-inputs context)]
    [value new-context]))
; TODO do the same for acc and nil

(defmulti write-value (fn [context idx] (get-slot-type context :outputs idx)))
(defmethod write-value :slot
  [context idx value]
  (let [
        i (dec idx)
        outputs (:outputs context)
        slot (nth outputs i)
        new-slot (sl/write-slot slot value)
        new-outputs (assoc i new-slot outputs)
        new-context (assoc :outputs new-outputs context)]
    new-context))
; TODO do the same for acc and nil
