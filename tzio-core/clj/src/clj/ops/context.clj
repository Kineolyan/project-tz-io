(ns clj.ops.context
  (:require [clj.refs.references :as refs]
            [clj.slots :as sl]))

(defn slot-type [_ [type & _]] type)

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
(defmethod can-write :slot)
(defmethod can-write :acc [_ _] true)
(defmethod can-write :nil [_ _] true)
