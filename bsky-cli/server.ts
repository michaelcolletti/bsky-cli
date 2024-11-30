// server.ts
import { Command } from "https://deno.land/x/cliffy@v0.25.7/command/mod.ts";
import { exec } from "https://deno.land/x/exec@0.0.5/mod.ts";

class BlueSkyService {
  private cliPath: string;

  constructor(cliPath: string) {
    this.cliPath = cliPath;
  }

  async post(message: string, options?: { visibility?: string }): Promise<void> {
    const command = [
      this.cliPath,
      'post',
      '--message',
      `"${message}"`,
      ...(options?.visibility ? [`--visibility`, options.visibility] : [])
    ];

    const { success, stdout, stderr } = await exec(command.join(' '));

    if (!success) {
      throw new Error(`Post failed: ${stderr}`);
    }

    console.log(stdout);
  }

  async read(options?: { limit?: number; timeRange?: string }): Promise<void> {
    const command = [
      this.cliPath,
      'read',
      ...(options?.limit ? [`--limit`, options.limit.toString()] : []),
      ...(options?.timeRange ? [`--time-range`, options.timeRange] : [])
    ];

    const { success, stdout, stderr } = await exec(command.join(' '));

    if (!success) {
      throw new Error(`Read failed: ${stderr}`);
    }

    console.log(stdout);
  }
}

async function main() {
  const command = new Command()
    .name("bluesky-cli")
    .version("0.1.0")
    .description("BlueSky CLI Wrapper")
    .command("post", "Post a message to BlueSky")
    .option("-m, --message <message:string>", "Message to post")
    .option("-v, --visibility <visibility:string>", "Post visibility")
    .action(async ({ message, visibility }) => {
      const service = new BlueSkyService('./bluesky-cli');
      await service.post(message, { visibility });
    })
    .command("read", "Read posts from BlueSky")
    .option("-l, --limit <limit:number>", "Number of posts to read", { default: 20 })
    .option("-t, --time-range <timeRange:string>", "Time range for posts")
    .action(async ({ limit, timeRange }) => {
      const service = new BlueSkyService('./bluesky-cli');
      await service.read({ limit, timeRange });
    });

  await command.parse(Deno.args);
}

if (import.meta.main) {
  main();
}