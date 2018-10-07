(ns clj.env-test
  (:require [clojure.test :refer :all]
            [clj.slot :refer :all]
            [clj.env :refer :all]))

(with-test
  (defn created-env [] 
    (create-env 4 [2] [0]))
  (is (= 
        (:slots (created-env)) 
        [
          (empty-slot)
          (empty-slot)
          (queue-slot)
          (empty-slot)]))
  (is (=
        (:nodes (created-env))
        (hash-map)))
  (is (=
        (:executions (created-env))
        (hash-map))))

(deftest test-new-node
  (testing "A new node"
    (testing "has the default initial value"
      (is (=
            (:acc (new-node 3))
            0)))
    (testing "has memory of the given size"
      (is (=
            (:memory (new-node 3))
            [0 0 0])))
    (testing "points to the first instruction"
      (is (=
            (:instruction (new-node 2))
            0)))))

(deftest test-new-execution
  (testing "A new execution"
    (testing "records the operations"
      (is (= 
            (:operations (new-execution [:in] [:out] [:op]))
            [:op])))
    (testing "records inputs"
      (is (=
            (:inputs (new-execution [:in] [:out] [:op]))
            [:in])))
    (testing "records outputs"
      (is (=
            (:outputs (new-execution [:in] [:out] [:op]))
            [:out])))))

(with-test
  (defn env-with-node []
    (add-node
      (create-env 4 [2] [0])
      "node-1"
      3
      [1] [2]
      [:op1 :op2]))
  (is (= 
        (get (:nodes (env-with-node)) "node-1") 
        (new-node 3)))
  (is (=
        (get (:executions (env-with-node)) "node-1")
        (new-execution [1] [2] [:op1 :op2]))))

(deftest test-input-slots
  (testing "extract input slots"
    (is
      (=
        (input-slots
          {
            :slots
            [
              (queue-slot 1)
              (empty-slot)
              (queue-slot)
              (data-slot 2)
              (queue-slot 3 4)]})
        [
          [0 (queue-slot 1)]
          [2 (queue-slot)]
          [4 (queue-slot 3 4)]]))))

(deftest test-consume
  (testing "consuming input data"
    (is
      (=
        (consume 
          {
            :slots 
            [
              (empty-slot)
              (queue-slot 1 2)
              (queue-slot)
              (empty-slot)]}
          [78 45])
        {
          :slots
          [
            (empty-slot)
            (queue-slot 1 2 78)
            (queue-slot 45)
            (empty-slot)]}))))

