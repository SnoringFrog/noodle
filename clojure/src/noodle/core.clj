(ns noodle.core
  (:gen-class :main true))

(defn history-line->command
  "Returns the command from a zsh_history-style line"
  [line]
  (nth (clojure.string/split line #"\s+") 2))

(defn weight
  "Returns a number representing how much time you've spent typing a command"
  [command invocation-count]
  (* invocation-count (count command)))

(defn sort-by-weight
  [m]
  (reverse (sort-by (partial apply weight) (vec m))))

(defn format-history-map
  [history-map]
  (letfn [(format-one [[command weight]]
            (format "%s: %d times\n" command weight))]
    (clojure.string/join (map format-one history-map))))

(defn format-history-lines
  [history-lines]
  (->> (map history-line->command history-lines)
       (frequencies)
       (sort-by-weight)
       (take 10)
       (format-history-map)))

(defn -main
  "Prints the top 10 most-costly commands and their frequencies"
  []
  (->
    *in*
    java.io.BufferedReader.
    line-seq
    format-history-lines
    print)
  (flush))
