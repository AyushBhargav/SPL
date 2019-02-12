import java.io.IOException;
import java.nio.charset.Charset;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Scanner;
import java.util.stream.Stream;

public class SPL {
    private static boolean hadError = false;

    static void error(int line, String message) {
        report(line, "", message);
    }

    private static void report(int line, String where, String message) {
        System.err.println(String.format("[line %d] Error %s : %s", line, where, message));
        hadError = true;
    }

    private static void runCode(String line) {

    }

    private static void runFile(String file) throws IOException {
        Stream<String> lines = Files.lines(Paths.get(file), Charset.defaultCharset());
        lines.forEachOrdered(SPL::runCode);
        if(hadError)
            System.exit(65);
    }

    private static void runREPL() {
        System.out.println("Welcome to SPL REPL shell.");
        Scanner in = new Scanner(System.in);
        //noinspection InfiniteLoopStatement
        while(true) {
            System.out.print("> ");
            runCode(in.nextLine());
            hadError = false;
        }
    }

    public static void main(String[] args) {
        if(args.length > 1) {
            System.out.println("Invalid command. Try: spl [filename.spl]");
            System.exit(64);
        }
        else if(args.length == 1) {
            // Filename is provided.
            try {
                runFile(args[0]);
            }
            catch(IOException e) {
                e.printStackTrace();
                System.exit(1);
            }
        }
        else {
            // REPL is to be run
            runREPL();
        }
    }
}
