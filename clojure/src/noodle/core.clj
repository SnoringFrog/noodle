(ns noodle.core)

(defn history-line->command
  "Returns the command name a zsh_history-style line"
  [line]
  (nth (clojure.string/split line #"\s+") 2))

(defn counts
  [commands]
  (frequencies commands))

(defn format-history-map
  [history-map]
  (letfn [(format-one [[command weight]]
            (format "%s: %d\n" command weight))]
    (clojure.string/join (map format-one history-map))))

(defn weight
  [m key1 key2]
  (compare [(* (get m key2) (count key2)) key2]
           [(* (get m key1) (count key1)) key1]))

(defn sort-by-weight
  [m]
  (into (sorted-map-by (partial weight m)) m))

(defn format-history-lines
  [history-lines]
  ((comp
     format-history-map
     (partial take 10)
     sort-by-weight
     counts
     (partial map history-line->command))
   history-lines))

(defn main
  []
  (->
    *in*
    java.io.BufferedReader.
    line-seq
    format-history-lines
    println))
