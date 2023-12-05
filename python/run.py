import db.helpers

import argparse
import threading
import logging 

logger = logging.getLogger(__name__)
logger.setLevel(logging.INFO)

fh = logging.FileHandler("bot.log")
fh.setLevel(logging.INFO)

ch = logging.StreamHandler()
ch.setLevel(logging.ERROR)

formatter = logging.Formatter('%(asctime)s - %(name)s - %(levelname)s - %(message)s')
fh.setFormatter(formatter)
ch.setFormatter(formatter)

logger.addHandler(fh)
logger.addHandler(ch)

def run_bot(bot, event):
    event.wait()
    bot.start(db)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="tiktok-botz")
    parser.add_argument("-t", "--type", type=str, help="bot type (like, comment, passive, control)", dest="bot_type", required=True)
    parser.add_argument("-c", "--collection-method", type=str, help="collection method (default / fyp)", dest="collection_method", default="default")
    parser.add_argument("-s", "--sessions", type=str, help="add sessionid cookies")
    args = parser.parse_args()

    db = db.helpers.AlgoproberDB(logger)
    bots = list()

    threads = []
    input_event = threading.Event()

    match args.bot_type:
        case "like":
            bots = db.get_bots(args.bot_type)
        case "comment":
            bots = db.get_bots(args.bot_type)
        case "passive":
            bots = db.get_bots(args.bot_type)
        case "control":
            bots = db.get_bots(args.bot_type)
        case default:
            print("[-] please enter a valid bot type: like, comment, passive, control")
            exit(1)
            
    for bot in bots:
        print(f"[+] Bot selected: {bot.email}:{bot.password} as type: {bot.type_} with status {bot.status}")
        
        if args.sessions:
            sessionid = input("[!] please enter sessionid cookie: ")
            db.add_sessionid(bot.username, sessionid)
            break
        else: 
            t = threading.Thread(target=run_bot, args=(bot, input_event)) 
            threads.append(t)
            t.start()
        
    if not args.sessions:
        input("[+] press enter to start bots: ")
        input_event.set()

        for t in threads:
            t.join()

