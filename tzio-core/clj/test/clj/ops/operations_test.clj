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

(deftest test-get-label
  (testing "get-label"
    (testing "extract the label"
      (is (= 
            (get-label [:label "lbl"]) 
            "lbl")))))

(deftest test-index
  (testing "index operations"
    (testing "without labels"
      (is 
        (= 
          (index [
                  [:neg]
                  [:sav 1]
                  [:jmp "start"]])
          [
            [
              [:neg]
              [:sav 1]
              [:jmp "start"]]
            {}])))
    (testing "with labels"
      (is 
        (= 
          (index [
                  [:label "start"]
                  [:neg]
                  [:label "middle"]
                  [:sav 1]
                  [:jmp "start"]
                  [:label "end"]])
          [
            [
              [:neg]
              [:sav 1]
              [:jmp "start"]]
            {
              "start" 0
              "middle" 1
              "end" 3}])))
    (testing "with consecutive labels"
      (is 
        (= 
          (index [
                  [:neg]
                  [:label "l1"]
                  [:label "l2"]
                  [:sav 1]
                  [:label "e1"]
                  [:label "e2"]
                  [:label "e3"]])
          [
            [
              [:neg]
              [:sav 1]]
            {
              "l1" 1
              "l2" 1
              "e1" 2
              "e2" 2
              "e3" 2}])))))
              
