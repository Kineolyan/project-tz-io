(ns clj.ops.processor
  (:require [clj.ops.operations :as ops]
            [clj.ops.context :as ctx]))

(defmulti run-cycle (fn [context [type & _]] type))
(defmethod run-cycle :mov
  [context [_ input output]]
  (let [
        readable (ctx/can-read context input)
        writable (ctx/can-write context output)]
    (if 
      (and readable writable)
      (->
        (ctx/read-value context input)
        (apply ctx/write-value))
      context)))

