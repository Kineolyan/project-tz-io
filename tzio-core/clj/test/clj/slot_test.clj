(ns clj.slot-test
  (:require [clojure.test :refer :all]
            [clj.slot :refer :all]))

(deftest test-is-queue
  (testing "#is-queue"
    (testing "with queue"
      (is (is-queue (queue-slot))))
    (testing "with empty slot"
      (is (not (is-queue (empty-slot)))))
    (testing "with data slot"
      (is (not (is-queue (data-slot 12)))))))

(deftest test-data-slot
  (testing "create data slot"
    (is (= (data-slot 13) [:slot 13]))))

(deftest test-queue-slot
  (testing "queue-slot"
    (testing "without value"
      (is 
        (=  
          (queue-slot)
          [:queue])))
    (testing "with one value"
      (is
        (=
          (queue-slot 13) 
          [:queue 13])))
    (testing "with many values"
      (is
        (=
          (queue-slot 23 4)
          [:queue 23 4])))))

(deftest test-read-slot
  (testing "read-slot"
    (testing "a data slot"
      (is
        (=
          (->
            (data-slot 23)
            (read-slot))
          [
            23
            (empty-slot)])))
    (testing "a queue"
      (is 
        (=
          (as->
            (queue-slot 12 5) q 
            (read-slot q))
          [
            12
            [:queue [5]]])))))

(deftest test-write-slot
  (testing "write-slot"
    (testing "an empty slot"
      (is
        (=
          (->
            (empty-slot)
            (write-slot 42))
          (data-slot 42))))
    (testing "a queue"
      (is (=
            (as-> 
              (queue-slot) q 
              (write-slot q 12) 
              (write-slot q 5))
            [:queue 12 5])))))
