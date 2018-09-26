(ns clj.env
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
  [^MovOpreation type]
  [:mov (refs/convert (.-input type) (.-output type))])

(defn sav
  [^SavOperation type]
  [:sav (.-slot type)])

(defn neg
  [_]
  [:neg])

(defn convert
  [type])


