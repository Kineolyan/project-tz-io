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
  [:mov (refs/convert-input (.-input type)) (refs/convert-output (.-output type))])

(defn sav
  [^SavOperation type]
  [:sav (.-slot type)])

(def swp
  [^SwpOperation type]
  [:swp (.-slot type)])

(defn add
  [^AddOperation type]
  [:add (refs/convert-input (.-input type))])

(defn sub
  [^SubOperation type]
  [:sub (refs/convert-input (.-input type))])

(defn neg
  [_]
  [:neg])

(defn lbl
  [^LabelOperation type]
  [:label (.-label type)])

(defn jmp
  [^JmpOperation type]
  [:jmp (.-label type)])

(defn jez
  [^JezOperation type]
  [:jez (.-label type)])

(defn jnz
  [^JnzOperation type]
  [:jnz (.-label type)])

(defn jgz
  [^JgzOperation type]
  [:jgz (.-label type)])

(defn jlz
  [^JlzOperation type]
  [:jlz (.-label type)])

(defn jro
  [^JroOperation type]
  [:jro (refs/convert-input (.-input type))])

(defn convert
  [type]
  (neg))

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
