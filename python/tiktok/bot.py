import random
import time

from enum import Enum
from bson.objectid import ObjectId
import os
from time import sleep
from seleniumwire import webdriver
from selenium.common.exceptions import ElementClickInterceptedException
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.common.action_chains import ActionChains
from webdriver_manager.chrome import ChromeDriverManager
from selenium_stealth import stealth

import db.helpers as videos

USER_AGENTS = [
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"
]

HASHTAGS = ["trumparrest", "donaldtrumpindicted", "donaldtrumparrested", "indictment", "donaldtrumpindictment", "trumpgoingtojail", "indictedwestand", "TrumpArraignment", "TrumpArraignmentDay", "HappyTrumpArraignmentDay", "TrumpIndicted", "presidentialrecordsact", "trumpespionage", "trumpobstruction", "trumpclassifieddocuments"]
TRUMP_COMMENTS = ["trump", "donaldtrump", "maga", "trump2024", "americafirst", "45thpresident", "trumptrain", "trumpsupporters", "trumppence"]
BIDEN_COMMENTS = ["biden", "joebiden", "bidenharris", "biden2024", "bidenadministration", "buildbackbetter", "democraticparty", "presidentbiden"]

def proxies() -> dict:
    wire_options = {
        "proxy": {
            "http": "http://proxy-cst.is.depaul.edu:3128",
            "https": "http://proxy-cst.is.depaul.edu:3128",
        }
    }

    return wire_options

def create_data_dir():
    dir_path = "data/html"
    if not os.path.exists(dir_path):
        os.makedirs(dir_path)
    else:
        pass


class AccountType(Enum):
    LIKE = 0
    COMMENT = 1
    PASSIVE = 2
    CONTROL = 3


class Bot():
    def __init__(self, id_: ObjectId, email: str, username: str, password: str, type_: str, sessionid: str, status: str, db):
        self.id_ = id_
        self.email = email
        self.username = username
        self.password = password
        self.type_ = type_
        self.status = status
        self.sessionid = sessionid
        self.driver = self.setup_webdriver()
        self.db = db

    def generate_account_type(self):
        options = [AccountType.LIKE, AccountType.COMMENT, AccountType.PASSIVE, AccountType.CONTROL]
        type_ = random.choice(options).value
        self.db.add_type_to_bot(self.username, type_)

        return type_


    def setup_webdriver(self):
        options = webdriver.ChromeOptions()
        options.headless = False
        options.add_argument(f"user-agent={random.choice(USER_AGENTS)}")
        options.add_argument("start-maximized")
        options.add_experimental_option("excludeSwitches", ["enable-automation"])
        options.add_experimental_option('useAutomationExtension', False)

        driver = webdriver.Chrome(
            ChromeDriverManager().install(), options=options #seleniumwire_options=proxies()
        )

        stealth(driver,
            languages=["en-US", "en"],
            vendor="Google Inc.",
            platform="Win32",
            webgl_vendor="Intel Inc.",
            renderer="Intel Iris OpenGL Engine",
            fix_hairline=True,
        )

        return driver

    # User action methods (login, scroll, like, comment)   
    def login(self):
        self.driver.get("https://www.tiktok.com/en")
        sessionid_cookie = {
                "name": "sessionid",
                "value": self.sessionid,
                "domain": ".tiktok.com",
                "path": "/",
                "secure": True,
                "httpOnly": True,
                "sameSite": "None"
            }

        self.driver.add_cookie(sessionid_cookie)
        self.driver.refresh()

    def scroll(self):
        actions = ActionChains(self.driver)
        actions.send_keys("\ue015")
        actions.perform()

    def like(self):
        actions = ActionChains(self.driver)
        actions.send_keys("L")
        actions.perform()

    def comment(self, logger, comment: str):
        comment_element = self.driver.find_element(By.XPATH, "//div[contains(@class, 'e1rzzhjk2') and @data-e2e='comment-input']")
        try:
            comment_element.click()
        except ElementClickInterceptedException:
            logger.error("captcha detected. refreshing page.")
            self.driver.refresh()

        time.sleep(1)
        text_field = self.driver.find_element(By.CLASS_NAME, "public-DraftStyleDefault-ltr")
        text_field.send_keys(comment + Keys.ENTER)

        time.sleep(10)

    # Navigation methods
    def search(self, query: str):
        self.driver.get(f"https://www.tiktok.com/search?q={query}")

    def goto_hashtag(self, hashtag: str):
        self.driver.get(f"https://www.tiktok.com/tag/{hashtag}")
        sleep(5)
        elements = self.driver.find_elements(By.TAG_NAME, "video")
        elements[0].click()

    def start(self, db):
        self.login()
        if self.driver.get_cookie("sessionid") is not None:
            db.logger.info(f"{self.username} logged into TikTok successfully")
            print(f"[+] logged in with {self.username}")

        else:
            print("cookie not found")
            return

        create_data_dir()

        start_time = time.time()
        db.logger.info(f"starting run on {self.username} @ {start_time}")
        print(f"[!] starting run on {self.username} @ {start_time}")

        if self.status == False:
            #self.type_ = self.generate_account_type()
            db.update_status(self.username, True)

        if self.type_ == 0 or self.type_ == 1:
            for i in range(len(HASHTAGS)-1):
                counter = 0
                self.goto_hashtag(HASHTAGS[i])
                sleep(5)
                while counter < 15:
                    author = self.driver.find_element(By.XPATH, "//span[contains(@data-e2e, 'browse-username')]").text
                    description = self.driver.find_element(By.XPATH, "//div[contains(@data-e2e, 'browse-video-desc')]").text
                    likes = self.driver.find_element(By.XPATH, "//strong[contains(@data-e2e, 'browse-like-count')]").text
                    comments = self.driver.find_element(By.XPATH, "//strong[contains(@data-e2e, 'browse-comment-count')]").text
                    bookmarks = self.driver.find_element(By.XPATH, "//strong[contains(@data-e2e, 'undefined-count')]").text
                    post_date = self.driver.find_element(By.XPATH, "//span[contains(@data-e2e, 'browser-nickname')]").text
                    video_sound = self.driver.find_element(By.XPATH, "//div[contains(@class, 'epjbyn3')]").text
                    video_url = self.driver.current_url

                    video = videos.Video(author, description, likes, comments, bookmarks, 0, post_date, video_sound, video_url)
                    db.insert_hashtag_video(self.username, video)
                    self.driver.save_screenshot(f"images/{self.username}+{video_url[45:]}-init.png")

                    sleep(5)

                    if self.type_ == 0:
                        self.like()
                        db.logger.info(f"{self.username} liked video {video_url}")
                        sleep(5)
                        
                        bookmark_elem = self.driver.find_element(By.XPATH, "//span[contains(@class, 'e1hk3hf91') and @data-e2e='undefined-icon']")
                        try:
                            bookmark_elem.click()
                            self.driver.save_screenshot(f"images/{self.username}+{video_url[45:]}-bookmark.png")
                            db.logger.info(f"{self.username} bookmarked {video_url}")
                        except ElementClickInterceptedException:
                            db.logger.error("captcha detected. refreshing page. attempting to restart current hashtag collection")
                            self.driver.refresh()
                            self.goto_hashtag(HASHTAGS[i])
                        except:
                            db.logger.error(f"{self.username} could not bookmark {video_url}")
                            self.driver.save_screenshot(f"images/{self.username}+{video_url[45:]}-bookmark-fail.png")
                    
                    else:
                        comment = random.choice(TRUMP_COMMENTS)
                        self.comment(db.logger, comment)
                        db.logger.info(f"{self.username} commented {comment} on {video_url}")

                    duration = random.randint(30,60)
                    sleep(duration)
                    counter += 1
                    
                    arrow = self.driver.find_element(By.XPATH, "//button[contains(@class, 'e11s2kul11') and @data-e2e='arrow-right']")
                    try:
                        arrow.click()
                    except ElementClickInterceptedException:
                        db.logger.error("captcha detected. refreshing page.")
                        self.driver.refresh()

            print("[SUCCESS] Run complete!")

        # passive account - scroll through 100 videos on the fyp and watch each video for 30-60 seconds
        elif self.type_ == 2:
            self.driver.get("https://www.tiktok.com/foryou")
            for i in range(100):
                duration = random.randint(30,50)
                sleep(5)
                try:
                    self.driver.find_element(By.TAG_NAME, "video").click()
                except ElementClickInterceptedException:
                    self.driver.refresh()
                    self.driver.find_element(By.TAG_NAME, "video").click()

                db.logger.info(f"{self.username} sleeping for {duration + 7}s")
                time.sleep(duration)

                author = self.driver.find_element(By.XPATH, "//span[contains(@data-e2e, 'browse-username')]").text
                description = self.driver.find_element(By.XPATH, "//div[contains(@data-e2e, 'browse-video-desc')]").text
                likes = self.driver.find_element(By.XPATH, "//strong[contains(@data-e2e, 'browse-like-count')]").text
                comments = self.driver.find_element(By.XPATH, "//strong[contains(@data-e2e, 'browse-comment-count')]").text
                bookmarks = self.driver.find_element(By.XPATH, "//strong[contains(@data-e2e, 'undefined-count')]").text
                shares = self.driver.find_element(By.XPATH, "//strong[contains(@data-e2e, 'share-count')]").text
                post_date = self.driver.find_element(By.XPATH, "//span[contains(@data-e2e, 'browser-nickname')]").text
                video_sound = self.driver.find_element(By.XPATH, "//div[contains(@class, 'epjbyn3')]").text
                video_url = self.driver.current_url

                #print(f"Current video: {author}, {description}, Likes: {likes}, Comments: {comments}, Bookmarks: {bookmarks}, Shares: {shares}, Post Date: {post_date}")
                video = videos.Video(author, description, likes, comments, bookmarks, shares, post_date, video_sound, video_url)
                db.insert_fyp_video(self.username, video)

                with open(f"data/html/{self.username}-{video_url.split('video/')[1]}.html", "w", encoding="utf-8") as f:
                    f.write(self.driver.page_source)

                self.driver.find_element(By.XPATH, "/html/body/div[1]/div[2]/div[4]/div/div[1]/button[1]").click()
                sleep(5)

                self.scroll()
                db.logger.info(f"[+] {self.username} scrolling on fyp")
            
        # do nothing since it is a control account
        elif self.type_ == 3:
            print("[-] control account. doing nothing")

        else:
            db.logger.error(f"[!] invalid user type! :: {self.type_}")

    def fyp(self, db):
        self.login()
        if self.driver.get_cookie("sessionid") is not None:
            db.logger.info(f"{self.username} logged into TikTok successfully")
            print(f"[+] logged in with {self.username}")

        else:
            print("cookie not found")
            return

        create_data_dir()

        start_time = time.time()
        db.logger.info(f"starting run on {self.username} @ {start_time}")
        print(f"[!] starting run on {self.username} @ {start_time}")


        self.driver.get("https://www.tiktok.com/foryou")
        for i in range(100):
            duration = random.randint(30,50)
            sleep(5)
            try:
                self.driver.find_element(By.TAG_NAME, "video").click()
            except ElementClickInterceptedException:
                self.driver.refresh()
                self.driver.find_element(By.TAG_NAME, "video").click()

            db.logger.info(f"{self.username} sleeping for {duration + 7}s")
            time.sleep(duration)

            author = self.driver.find_element(By.XPATH, "//span[contains(@data-e2e, 'browse-username')]").text
            description = self.driver.find_element(By.XPATH, "//div[contains(@data-e2e, 'browse-video-desc')]").text
            likes = self.driver.find_element(By.XPATH, "//strong[contains(@data-e2e, 'browse-like-count')]").text
            comments = self.driver.find_element(By.XPATH, "//strong[contains(@data-e2e, 'browse-comment-count')]").text
            bookmarks = self.driver.find_element(By.XPATH, "//strong[contains(@data-e2e, 'undefined-count')]").text
            shares = self.driver.find_element(By.XPATH, "//strong[contains(@data-e2e, 'share-count')]").text
            post_date = self.driver.find_element(By.XPATH, "//span[contains(@data-e2e, 'browser-nickname')]").text
            video_sound = self.driver.find_element(By.XPATH, "//div[contains(@class, 'epjbyn3')]").text
            video_url = self.driver.current_url

            #print(f"Current video: {author}, {description}, Likes: {likes}, Comments: {comments}, Bookmarks: {bookmarks}, Shares: {shares}, Post Date: {post_date}")
            video = videos.Video(author, description, likes, comments, bookmarks, shares, post_date, video_sound, video_url)
            db.insert_fyp_video(self.username, video)

            with open(f"data/html/{self.username}-{video_url.split('video/')[1]}.html", "w", encoding="utf-8") as f:
                f.write(self.driver.page_source)

            self.driver.find_element(By.XPATH, "/html/body/div[1]/div[2]/div[4]/div/div[1]/button[1]").click()
            sleep(5)

            self.scroll()
            db.logger.info(f"[+] {self.username} scrolling on fyp")
