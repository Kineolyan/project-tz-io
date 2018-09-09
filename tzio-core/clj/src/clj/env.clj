(ns clj.env
  [:require clj.slot])

(defn create-env
  "Create a blank environment"
  [slot-count input-ids output-ids]
  (let 
    [
      indexes (range 1 slot-count)
      is-input (fn [i] (some #(= i %) input-ids))
      slots (map #(if (is-input %) (queue-slot) (empty-slot)))] 
    {
      :slots slots
      :nodes []
      :executions []}))

(defn add)
