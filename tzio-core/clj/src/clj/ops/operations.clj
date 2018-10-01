(ns clj.ops.operations
  (:require [clj.refs.references :as refs])
  (:import 
    (com.kineolyan.tzio.v1.api.ops
        MovOperation
        SavOperation
        SwpOperation 
        AddOperation 
        SubOperation 
        NegOperation 
        LabelOperation 
        JmpOperation 
        JezOperation 
        JnzOperation 
        JlzOperation 
        JgzOperation 
        JroOperation)))
  
(defn mov
  [^MovOperation type]
  [:mov (refs/convert (.-input type) (.-output type))])

(defn sav
  [^SavOperation type]
  [:sav (.-slot type)])

(defn neg
  [_]
  [:neg])

(defn lbl
  [^LabelOperation type]
  [:label (.-label type)])

(defn convert
  [type])

(defmulti is-label? (fn [[type & remaining]] type))
(defmethod is-label? :label [& _] true)
(defmethod is-label? :default [& _] false)

(defn get-label [[_ lbl]] lbl)

(defn index
  [operations]
  (let
    [
      [ops idx] 
      (reduce 
        (fn [[ops idx] op]
          (if 
            (is-label? op)
            [
              ops
              (assoc! idx (get-label op) (count ops))]
            [
              (conj! ops op)
              idx]))
        [(transient []) (transient {})]
        operations)]
    [
      (persistent! ops)
      (persistent! idx)]))
