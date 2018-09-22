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
