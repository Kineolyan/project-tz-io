(ns clj.env
  [:require clj.slot])

(defn create-env
  "Create a blank environment"
  [slot-count input-ids output-ids]
  (let [slots (map #(queue-slot) input-ids))] 
    {
      :slots []
      :nodes []
      :executions []}))

(defn add)
