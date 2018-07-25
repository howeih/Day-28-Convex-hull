#Day 28: Convex hull
<br>
Graham's scan is a method of finding the convex hull of a finite set of points in the plane with time complexity O(n log n). It is named after Ronald Graham, who published the original algorithm in 1972.[1] The algorithm finds all vertices of the convex hull ordered along its boundary. It uses a stack to detect and remove concavities in the boundary efficiently.
<br>
graph:

```
    let mut points = vec![
        Point { x: 0., y: 3. },
        Point { x: 1., y: 1. },
        Point { x: 2., y: 2. },
        Point { x: 4., y: 4. },
        Point { x: 0., y: 0. },
        Point { x: 1., y: 2. },
        Point { x: 3., y: 1. },
        Point { x: 3., y: 3. },
    ];
```

run:

```
[Point { x: 0.0, y: 0.0 }, Point { x: 3.0, y: 1.0 }, Point { x: 4.0, y: 4.0 }, Point { x: 0.0, y: 3.0 }]
```
