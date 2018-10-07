(ns clj.env
  (:require [clj.slot :as sl]))

(defn create-env
  "Create a blank environment"
  [slot-count input-ids output-ids]
  (let
    [
      indexes (range 0 slot-count)
      is-input (fn [i] (some #(= i %) input-ids))
      slots (map
                  #(if (is-input %) (sl/queue-slot) (sl/empty-slot))
                  indexes)]
    {
      :slots slots
      :nodes (hash-map)
      :executions (hash-map)}))

(defn new-node
  "Creates a new node, with all the state information about the node"
  [memory]
  {
    :acc 0
    :memory (repeat memory 0)
    :instruction 0})

(defn new-execution
  "Create a new execution for a node"
  [inputs outputs operations]
  {
    :operations operations
    :inputs inputs
    :outputs outputs})

(defn add-node
  "Adds a node to the existing system"
  [env name memory inputs outputs operations]
  (let
    [
      nodes (:nodes env)
      executions (:executions env)
      new-nodes (assoc nodes name (new-node memory))
      new-executions (assoc executions name (new-execution inputs outputs operations))]
    (assoc env
      :nodes new-nodes
      :executions new-executions)))

(defn input-slots
  "Gets the indexes of environment inputs"
  [env]
  (let 
    [
      slots (:slots env)
      slot-indexes (range (count slots))
      indexes (filter #(sl/is-queue (nth slots %)) slot-indexes)]
    (map (fn [i] [i (nth slots i)]) indexes)))

(defn consume
  "Feeds the environment with external data"
  [env data]
  (let
    [
      inputs (input-slots env)
      to-update (map vector inputs data)
      fed-slots (map
                  (fn
                    [[[idx slot] value]]
                    [
                      idx
                      (sl/write-slot slot value)])
                  to-update)
      updated-slots (reduce
                      (fn [acc [idx slot]] (assoc! acc idx slot))
                      (transient (:slots env))
                      fed-slots)]
    (assoc env :slots (persistent! updated-slots))))
