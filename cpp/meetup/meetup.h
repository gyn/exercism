//
//
//
#include <boost/date_time/gregorian/gregorian.hpp>

namespace meetup {
    class scheduler {
    public:
        scheduler(boost::gregorian::date::month_type month, boost::gregorian::date::year_type year);

    public:
        boost::gregorian::date monteenth() const;
        boost::gregorian::date tuesteenth() const;
        boost::gregorian::date wednesteenth() const;
        boost::gregorian::date thursteenth() const;
        boost::gregorian::date friteenth() const;
        boost::gregorian::date saturteenth() const;
        boost::gregorian::date sunteenth() const;

        boost::gregorian::date first_monday() const;
        boost::gregorian::date first_tuesday() const;
        boost::gregorian::date first_wednesday() const;
        boost::gregorian::date first_thursday() const;
        boost::gregorian::date first_friday() const;
        boost::gregorian::date first_saturday() const;
        boost::gregorian::date first_sunday() const;

        boost::gregorian::date second_monday() const;
        boost::gregorian::date second_tuesday() const;
        boost::gregorian::date second_wednesday() const;
        boost::gregorian::date second_thursday() const;
        boost::gregorian::date second_friday() const;
        boost::gregorian::date second_saturday() const;
        boost::gregorian::date second_sunday() const;

        boost::gregorian::date third_monday() const;
        boost::gregorian::date third_tuesday() const;
        boost::gregorian::date third_wednesday() const;
        boost::gregorian::date third_thursday() const;
        boost::gregorian::date third_friday() const;
        boost::gregorian::date third_saturday() const;
        boost::gregorian::date third_sunday() const;

        boost::gregorian::date fourth_monday() const;
        boost::gregorian::date fourth_tuesday() const;
        boost::gregorian::date fourth_wednesday() const;
        boost::gregorian::date fourth_thursday() const;
        boost::gregorian::date fourth_friday() const;
        boost::gregorian::date fourth_saturday() const;
        boost::gregorian::date fourth_sunday() const;

        boost::gregorian::date last_monday() const;
        boost::gregorian::date last_tuesday() const;
        boost::gregorian::date last_wednesday() const;
        boost::gregorian::date last_thursday() const;
        boost::gregorian::date last_friday() const;
        boost::gregorian::date last_saturday() const;
        boost::gregorian::date last_sunday() const;

    private:
        boost::gregorian::date teenth_weekday(boost::date_time::weekdays day) const;
        boost::gregorian::date first_weekday(boost::date_time::weekdays day) const;
        boost::gregorian::date second_weekday(boost::date_time::weekdays day) const;
        boost::gregorian::date third_weekday(boost::date_time::weekdays day) const;
        boost::gregorian::date fourth_weekday(boost::date_time::weekdays day) const;
        boost::gregorian::date last_weekday(boost::date_time::weekdays day) const;
        boost::gregorian::date nth_weekday(boost::gregorian::nth_day_of_the_week_in_month::week_num n,
                                           boost::date_time::weekdays day) const;

    private:
        const boost::gregorian::date::year_type year;
        const boost::gregorian::date::month_type month;
    };

    scheduler::scheduler(boost::gregorian::date::month_type month, boost::gregorian::date::year_type year)
            : year(year), month(month) {
    }

    //
    //
    //
    boost::gregorian::date scheduler::teenth_weekday(boost::date_time::weekdays day) const {
        auto date = boost::gregorian::first_day_of_the_week_after(day);
        return date.get_date({year, month, 12});
    }

    boost::gregorian::date scheduler::monteenth() const {
        return teenth_weekday(boost::date_time::weekdays::Monday);
    }

    boost::gregorian::date scheduler::tuesteenth() const {
        return teenth_weekday(boost::date_time::weekdays::Tuesday);
    }

    boost::gregorian::date scheduler::wednesteenth() const {
        return teenth_weekday(boost::date_time::weekdays::Wednesday);
    }

    boost::gregorian::date scheduler::thursteenth() const {
        return teenth_weekday(boost::date_time::weekdays::Thursday);
    }

    boost::gregorian::date scheduler::friteenth() const {
        return teenth_weekday(boost::date_time::weekdays::Friday);
    }

    boost::gregorian::date scheduler::saturteenth() const {
        return teenth_weekday(boost::date_time::weekdays::Saturday);
    }

    boost::gregorian::date scheduler::sunteenth() const {
        return teenth_weekday(boost::date_time::weekdays::Sunday);
    }

    //
    //
    //
    boost::gregorian::date scheduler::first_weekday(boost::date_time::weekdays day) const {
        auto date = boost::gregorian::first_day_of_the_week_in_month(day, month);
        return date.get_date(year);
    }

    boost::gregorian::date scheduler::first_monday() const {
        return first_weekday(boost::date_time::weekdays::Monday);
    }

    boost::gregorian::date scheduler::first_tuesday() const {
        return first_weekday(boost::date_time::weekdays::Tuesday);
    }

    boost::gregorian::date scheduler::first_wednesday() const {
        return first_weekday(boost::date_time::weekdays::Wednesday);
    }

    boost::gregorian::date scheduler::first_thursday() const {
        return first_weekday(boost::date_time::weekdays::Thursday);
    }

    boost::gregorian::date scheduler::first_friday() const {
        return first_weekday(boost::date_time::weekdays::Friday);
    }

    boost::gregorian::date scheduler::first_saturday() const {
        return first_weekday(boost::date_time::weekdays::Saturday);
    }

    boost::gregorian::date scheduler::first_sunday() const {
        return first_weekday(boost::date_time::weekdays::Sunday);
    }

    //
    //
    //
    boost::gregorian::date scheduler::nth_weekday(boost::gregorian::nth_day_of_the_week_in_month::week_num n,
                                                  boost::date_time::weekdays day) const {
        auto date = boost::gregorian::nth_day_of_the_week_in_month(n, day, month);
        return date.get_date(year);
    }

    //
    //
    //
    boost::gregorian::date scheduler::second_weekday(boost::date_time::weekdays day) const {
        return nth_weekday(boost::gregorian::nth_day_of_the_week_in_month::second, day);
    }

    boost::gregorian::date scheduler::second_monday() const {
        return second_weekday(boost::date_time::weekdays::Monday);
    }

    boost::gregorian::date scheduler::second_tuesday() const {
        return second_weekday(boost::date_time::weekdays::Tuesday);
    }

    boost::gregorian::date scheduler::second_wednesday() const {
        return second_weekday(boost::date_time::weekdays::Wednesday);
    }

    boost::gregorian::date scheduler::second_thursday() const {
        return second_weekday(boost::date_time::weekdays::Thursday);
    }

    boost::gregorian::date scheduler::second_friday() const {
        return second_weekday(boost::date_time::weekdays::Friday);
    }

    boost::gregorian::date scheduler::second_saturday() const {
        return second_weekday(boost::date_time::weekdays::Saturday);
    }

    boost::gregorian::date scheduler::second_sunday() const {
        return second_weekday(boost::date_time::weekdays::Sunday);
    }

    //
    //
    //
    boost::gregorian::date scheduler::third_weekday(boost::date_time::weekdays day) const {
        return nth_weekday(boost::gregorian::nth_day_of_the_week_in_month::third, day);
    }

    boost::gregorian::date scheduler::third_monday() const {
        return third_weekday(boost::date_time::weekdays::Monday);
    }

    boost::gregorian::date scheduler::third_tuesday() const {
        return third_weekday(boost::date_time::weekdays::Tuesday);
    }

    boost::gregorian::date scheduler::third_wednesday() const {
        return third_weekday(boost::date_time::weekdays::Wednesday);
    }

    boost::gregorian::date scheduler::third_thursday() const {
        return third_weekday(boost::date_time::weekdays::Thursday);
    }

    boost::gregorian::date scheduler::third_friday() const {
        return third_weekday(boost::date_time::weekdays::Friday);
    }

    boost::gregorian::date scheduler::third_saturday() const {
        return third_weekday(boost::date_time::weekdays::Saturday);
    }

    boost::gregorian::date scheduler::third_sunday() const {
        return third_weekday(boost::date_time::weekdays::Sunday);
    }

    //
    //
    //
    boost::gregorian::date scheduler::fourth_weekday(boost::date_time::weekdays day) const {
        return nth_weekday(boost::gregorian::nth_day_of_the_week_in_month::fourth, day);
    }

    boost::gregorian::date scheduler::fourth_monday() const {
        return fourth_weekday(boost::date_time::weekdays::Monday);
    }

    boost::gregorian::date scheduler::fourth_tuesday() const {
        return fourth_weekday(boost::date_time::weekdays::Tuesday);
    }

    boost::gregorian::date scheduler::fourth_wednesday() const {
        return fourth_weekday(boost::date_time::weekdays::Wednesday);
    }

    boost::gregorian::date scheduler::fourth_thursday() const {
        return fourth_weekday(boost::date_time::weekdays::Thursday);
    }

    boost::gregorian::date scheduler::fourth_friday() const {
        return fourth_weekday(boost::date_time::weekdays::Friday);
    }

    boost::gregorian::date scheduler::fourth_saturday() const {
        return fourth_weekday(boost::date_time::weekdays::Saturday);
    }

    boost::gregorian::date scheduler::fourth_sunday() const {
        return fourth_weekday(boost::date_time::weekdays::Sunday);
    }

    //
    //
    //
    boost::gregorian::date scheduler::last_weekday(boost::date_time::weekdays day) const {
        return boost::gregorian::last_day_of_the_week_in_month(day, month).get_date(year);
    }

    boost::gregorian::date scheduler::last_monday() const {
        return last_weekday(boost::date_time::weekdays::Monday);
    }

    boost::gregorian::date scheduler::last_tuesday() const {
        return last_weekday(boost::date_time::weekdays::Tuesday);
    }

    boost::gregorian::date scheduler::last_wednesday() const {
        return last_weekday(boost::date_time::weekdays::Wednesday);
    }

    boost::gregorian::date scheduler::last_thursday() const {
        return last_weekday(boost::date_time::weekdays::Thursday);
    }

    boost::gregorian::date scheduler::last_friday() const {
        return last_weekday(boost::date_time::weekdays::Friday);
    }

    boost::gregorian::date scheduler::last_saturday() const {
        return last_weekday(boost::date_time::weekdays::Saturday);
    }

    boost::gregorian::date scheduler::last_sunday() const {
        return last_weekday(boost::date_time::weekdays::Sunday);
    }
}