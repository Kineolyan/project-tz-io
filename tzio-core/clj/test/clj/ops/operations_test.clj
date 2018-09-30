(ns clj.ops.operations-test
  (:require 
    [clojure.test :refer :all]
    [clj.ops.operations :refer :all]))

(deftest test-is-label
  (testing "is-label?"
    (testing "on label"
      (is (= 
            (is-label? [:label "value"])
            true)))
    (testing "on others"
      (is (=
            (is-label? [:neg])
            false)))))

(deftest test-index
  (testing "index operations"
    (testing "without labels"
      (is (= true false)))))
