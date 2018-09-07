; (ns com.kineolyan.tzio.v1.clj
;   (:import [com.kineolyan.tzio.v1.api.arch TzSystem]))

; (defn reify-doer
;   "Some docstring about what this specific implementation of Doer
;   does differently than the other ones. For example, this one does
;   not actually do anything but print the given string to stdout."
;   []
;   (reify
;     Doer
;     (doSomethin [this in] (println in))))

(ns com.kineolyan.tzio.v1.clj.system
  (:gen-class
   :implements [com.kineolyan.tzio.v1.api.arch.TzSystem]
   :init init
   :constructors {[] []}))

(defn -init []
  [[]])

(defn -createEnv [this]
  []pl)
