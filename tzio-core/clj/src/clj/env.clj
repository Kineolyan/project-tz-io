(ns clj.env
  (:require [clj.slot as sl]))

(defn create-env
  "Create a blank environment"
  [slot-count input-ids output-ids]
  (let 
    [
      indexes (range 1 slot-count)
      is-input (fn [i] (some #(= i %) input-ids))
      slots (map #(if (is-input %) (sl/queue-slot) (sl/empty-slot)))] 
    {
      :slots slots
      :nodes (hash-map)
      :executions (hash-map)}))

(defn add-node
  "Adds a node to the existing system"
  [env name memory inputs outputs operations]
  (let 
    [
      nodes (:nodes env)
      executions (:executions env)
      new-nodes (assoc! nodes name (new-node memory inputs outputs))
      new-executions (assoc! executions name (new-execution operations))])
  (assoc! env 
    :nodes new-nodes 
    :executions new-executions))

(defn new-node
  "Creates a new node"
  [name memory inputs outputs]
  {
    :memory (repeat memory 0)
    :inputs inputs
    :outputs outputs})

(defn new-execution
  "Create a new execution node"
  [operations]
  {
    :operations operations
    :index 0})

(defn consume
  "Feeds the environment with external data"
  [env data]
  (let
    [
      slots (:slots env)
      idx-count (count slots)
      indexes (filter #(sl/is-queue %) slots)
      to-update (map vector indexes data)
      fed-slots ()
        (reduce
          (fn [s (idx value)]
            (assoc! s idx value))
          slots
          to-update)])  
  (assoc! env :slots fed-slots))
