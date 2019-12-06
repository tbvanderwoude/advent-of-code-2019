import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.*;
import java.util.regex.Pattern;

public class Main {
    public static class Node
    {
        String name;
        Node parent;
        public Node(Node parent, String name)
        {
            this.name=name;
            this.parent=parent;
            children=new ArrayList();
        }
        public void addChild(Node child) {
            children.add(child);
        }
        List<Node> children;
    }
    public static int getOrbits(Node n, int indirect)
    {
        int total=indirect;
        for(Node child : n.children)
        {
            total+=getOrbits(child,indirect+1);
        }
        return total;
    }
    public static void main(String[] args) {
        BufferedReader reader;
        Map<String, Node> nodes=new HashMap();
        try {
            reader = new BufferedReader(new FileReader(
                    "universe.txt"));
            String line = reader.readLine();
            while (line != null) {
                String[] planets = line.split(Pattern.quote(")"));
                Node a = nodes.get(planets[0]);
                Node b = nodes.get(planets[1]);
                if (a == null) {
                    a = new Node(null,planets[0]);
                    nodes.put(planets[0], a);
                }
                if (b == null) {
                    b = new Node(a,planets[1]);
                    nodes.put(planets[1], b);
                }
                a.addChild(b);
                b.parent=a;
                line = reader.readLine();
            }
            Map.Entry<String, Node> root = null;
            for (Map.Entry<String, Node> e : nodes.entrySet()) {
                if (e.getValue().parent == null) {
                    root = e;
                } else if (e.getValue().parent == null && root != null) {
                    System.out.println("Something went wrong parsing");
                }
            }
            System.out.println("The root is now " + root.getKey());
            System.out.println(getOrbits(nodes.get("COM"), 0));

            Node santaOrbit = nodes.get("SAN").parent;
            Node myOrbit = nodes.get("YOU").parent;
            if(santaOrbit==null||myOrbit==null)
            {
                System.out.println("Either orbit was null");
            }
            //BFS TIME BABY
            Stack<Node> santaGraph=new Stack<>();
            Node n=santaOrbit;
            while(n!=null)
            {
                santaGraph.push(n);
                n=n.parent;
            }
            Stack<Node> myGraph=new Stack<>();
            n=myOrbit;
            while(n!=null)
            {
                myGraph.push(n);
                n=n.parent;
            }
            System.out.println(root.getKey()+" and "+santaGraph.peek().name);

            while(santaGraph.peek()==myGraph.peek())
            {
                santaGraph.pop();
                myGraph.pop();
            }
            Iterator<Node> value = myGraph.iterator();
            System.out.println("The iterator values are: ");
            while (value.hasNext()) {
                System.out.println(value.next().name);
            }
            value = santaGraph.iterator();
            System.out.println("The iterator values are: ");
            while (value.hasNext()) {
                System.out.println(value.next().name);
            }
            int dist = myGraph.size()+santaGraph.size();
            System.out.println(dist);
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
